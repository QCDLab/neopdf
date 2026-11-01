#include <assert.h>
#include <math.h>
#include <neopdf_capi.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

/* An arithmetic on `long double` scalars, allocated on the heap. The context
 * counts the scalars that are alive, and can make `ln` fail. */
typedef struct {
    long live;
    bool fail_ln;
} Context;

static int new_scalar(void *context, long double value, void **out) {
    long double *scalar = (long double *)malloc(sizeof(long double));
    if (scalar == NULL) { return 1; }
    *scalar                     = value;
    ((Context *)context)->live += 1;
    *out                        = scalar;
    return 0;
}

static long double get(const void *a) { return *(const long double *)a; }

static int         lift(void *context, double value, void **out) {
    return new_scalar(context, (long double)value, out);
}

static int clone_scalar(void *context, const void *a, void **out) {
    return new_scalar(context, get(a), out);
}

static void release(void *context, void *a) {
    ((Context *)context)->live -= 1;
    free(a);
}

static int add(void *context, const void *a, const void *b, void **out) {
    return new_scalar(context, get(a) + get(b), out);
}

static int sub(void *context, const void *a, const void *b, void **out) {
    return new_scalar(context, get(a) - get(b), out);
}

static int mul(void *context, const void *a, const void *b, void **out) {
    return new_scalar(context, get(a) * get(b), out);
}

static int divide(void *context, const void *a, const void *b, void **out) {
    return new_scalar(context, get(a) / get(b), out);
}

static int ln(void *context, const void *a, void **out) {
    if (((Context *)context)->fail_ln) { return 1; }
    return new_scalar(context, logl(get(a)), out);
}

static int
compare(void *context, const void *a, const void *b, NeopdfOrdering *out) {
    (void)context;
    long double x = get(a), y = get(b);
    *out = x < y    ? NEOPDF_ORDERING_LESS
           : x > y  ? NEOPDF_ORDERING_GREATER
           : x == y ? NEOPDF_ORDERING_EQUAL
                    : NEOPDF_ORDERING_UNORDERED;
    return 0;
}

static int is_finite(void *context, const void *a, bool *out) {
    (void)context;
    *out = isfinite(get(a));
    return 0;
}

static NeoPDFArithmetic long_double_arithmetic(Context *context) {
    NeoPDFArithmetic arithmetic = {
        context,
        lift,
        clone_scalar,
        release,
        add,
        sub,
        mul,
        divide,
        ln,
        compare,
        is_finite,
    };
    return arithmetic;
}

/* Evaluates `xfxq2` in `long double`, or returns NaN and stores the error code.
 */
static long double xfxq2(const NeoPDFPrecisionPDF *view,
    Context                                       *context,
    int                                            pid,
    long double                                    x,
    long double                                    q2,
    NeopdfResult                                  *result) {
    void *value = NULL;
    *result     = neopdf_precision_pdf_xfxq2(view, pid, &x, &q2, &value);
    if (*result != NEOPDF_RESULT_SUCCESS) {
        assert(value == NULL);
        return NAN;
    }
    long double xf = get(value);
    release(context, value); /* the caller owns the result */
    return xf;
}

void test_reproduces_xfxq2() {
    printf("=== Test the long double view against the double API ===\n");

    NeoPDFWrapper      *pdf     = neopdf_pdf_load("NNPDF40_nnlo_as_01180", 0);
    Context             context = {0, false};
    NeoPDFArithmetic    arithmetic = long_double_arithmetic(&context);
    NeoPDFPrecisionPDF *view       = NULL;
    NeopdfResult result = neopdf_pdf_with_arithmetic(pdf, &arithmetic, &view);
    assert(result == NEOPDF_RESULT_SUCCESS);
    assert(view != NULL);

    int    pids[]   = {-5, -2, -1, 1, 2, 4, 21};
    int    num_pids = sizeof(pids) / sizeof(pids[0]);
    double xs[]     = {1e-7, 1e-4, 1e-2, 0.3, 0.8};
    double q2s[]    = {3.0, 25.0, 1e4, 1e6};

    printf("%6s %12s %12s %20s\n", "pid", "x", "Q2", "long double");
    printf("------------------------------------------------------\n");
    for (int i = 0; i < num_pids; ++i) {
        for (int j = 0; j < 5; ++j) {
            for (int k = 0; k < 4; ++k) {
                long double xf =
                    xfxq2(view, &context, pids[i], xs[j], q2s[k], &result);
                assert(result == NEOPDF_RESULT_SUCCESS);
                double reference =
                    neopdf_pdf_xfxq2(pdf, pids[i], xs[j], q2s[k]);
                assert(
                    fabsl(xf - reference) <= 1e-13 * fabs(reference) + 1e-300);
                printf("%6d %12.4e %12.4e %20.12Le\n",
                    pids[i],
                    xs[j],
                    q2s[k],
                    xf);
            }
        }
    }

    /* A flavour that the set does not contain is zero. */
    assert(xfxq2(view, &context, 6, 0.1L, 100.0L, &result) == 0.0L);
    assert(result == NEOPDF_RESULT_SUCCESS);

    long double q2     = 8315.0L;
    void       *alphas = NULL;
    result             = neopdf_precision_pdf_alphas_q2(view, &q2, &alphas);
    assert(result == NEOPDF_RESULT_SUCCESS);
    assert(fabsl(get(alphas) - neopdf_pdf_alphas_q2(pdf, 8315.0)) <= 1e-15);
    printf("alpha_s(Q2 = 8315) = %.12Le\n", get(alphas));
    release(&context, alphas);

    /* The view does not refer to the PDF once it is prepared. */
    neopdf_pdf_free(pdf);
    assert(xfxq2(view, &context, 21, 0.1L, 100.0L, &result) > 0.0L);

    neopdf_precision_pdf_free(view);
    /* Every scalar created by NeoPDF has been released. */
    assert(context.live == 0);
    printf("\n");
}

void test_errors() {
    printf("=== Test the errors of the view ===\n");

    NeoPDFWrapper      *pdf     = neopdf_pdf_load("NNPDF40_nnlo_as_01180", 0);
    Context             context = {0, false};
    NeoPDFArithmetic    arithmetic = long_double_arithmetic(&context);
    NeoPDFPrecisionPDF *view       = NULL;
    assert(neopdf_pdf_with_arithmetic(pdf, &arithmetic, &view) ==
           NEOPDF_RESULT_SUCCESS);

    NeopdfResult result;
    /* There is no extrapolation. */
    xfxq2(view, &context, 21, 1e-12L, 100.0L, &result);
    printf("x outside the grid:   %d\n", result);
    xfxq2(view, &context, 21, 0.1L, 1.0L, &result);
    printf("Q2 outside the grid:  %d\n", result);
    xfxq2(view, &context, 21, (long double)NAN, 100.0L, &result);
    printf("non-finite x:         %d\n", result);
    xfxq2(view, &context, 21, 0.1L, 100.0L, &result);
    printf("inside the grid:      %d\n", result);
    neopdf_precision_pdf_free(view);
    assert(context.live == 0);

    /* A failing callback aborts the preparation, without leaking scalars. */
    context.fail_ln = true;
    result          = neopdf_pdf_with_arithmetic(pdf, &arithmetic, &view);
    printf("failing callback:     %d\n", result);
    assert(view == NULL);
    assert(context.live == 0);

    /* A missing callback is a null pointer. */
    arithmetic.ln = NULL;
    printf("missing callback:     %d\n",
        neopdf_pdf_with_arithmetic(pdf, &arithmetic, &view));
    neopdf_pdf_free(pdf);

    /* Only collinear `LogBicubic` sets are supported. */
    NeoPDFWrapper *tmd =
        neopdf_pdf_load("MAP22_grids_FF_Km_N3LL.neopdf.lz4", 0);
    arithmetic = long_double_arithmetic(&context);
    printf("unsupported set:      %d\n",
        neopdf_pdf_with_arithmetic(tmd, &arithmetic, &view));
    assert(context.live == 0);
    neopdf_pdf_free(tmd);
}

int main() {
    test_reproduces_xfxq2();
    test_errors();
    return EXIT_SUCCESS;
}
