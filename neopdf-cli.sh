#!/bin/bash

# Gum-based interactive wrapper for NeoPDF CLI

# Ensure gum is installed
if ! command -v gum &> /dev/null
then
    echo "gum could not be found. Please install it from https://github.com/charmbracelet/gum"
    exit
fi

# --- Main Menu ---
echo "Welcome to the NeoPDF interactive CLI!"
COMMAND=$(gum choose "read" "compute" "write" "install")

# --- Read Subcommands ---
if [ "$COMMAND" == "read" ]; then
    READ_COMMAND=$(gum choose "metadata" "num_subgrids" "subgrid-info" "subgrid" "git-version")

    PDF_NAME=$(gum input --placeholder "Enter PDF name")

    case $READ_COMMAND in
        "metadata")
            neopdf read metadata "$PDF_NAME"
            ;;
        "num_subgrids")
            neopdf read num_subgrids "$PDF_NAME"
            ;;
        "subgrid-info")
            MEMBER=$(gum input --placeholder "Enter member index")
            SUBGRID_INDEX=$(gum input --placeholder "Enter subgrid index")
            neopdf read subgrid-info "$PDF_NAME" -m "$MEMBER" -s "$SUBGRID_INDEX"
            ;;
        "subgrid")
            MEMBER=$(gum input --placeholder "Enter member index")
            SUBGRID_INDEX=$(gum input --placeholder "Enter subgrid index")
            PID=$(gum input --placeholder "Enter Parton PID")
            NUCLEON_INDEX=$(gum input --placeholder "Enter nucleon index (0 if N/A)")
            ALPHAS_INDEX=$(gum input --placeholder "Enter alpha_s index (0 if N/A)")
            KT_INDEX=$(gum input --placeholder "Enter kT index (0 if N/A)")
            neopdf read subgrid "$PDF_NAME" -m "$MEMBER" -s "$SUBGRID_INDEX" --pid="$PID" --nucleon-index "$NUCLEON_INDEX" --alphas-index "$ALPHAS_INDEX" --kt-index "$KT_INDEX"
            ;;
        "git-version")
            neopdf read git-version "$PDF_NAME"
            ;;
    esac
fi

# --- Compute Subcommands ---
if [ "$COMMAND" == "compute" ]; then
    COMPUTE_COMMAND=$(gum choose "xfx_q2" "alphas_q2" "xfx_q2_kt")

    PDF_NAME=$(gum input --placeholder "Enter PDF name")
    MEMBER=$(gum input --placeholder "Enter member index")

    case $COMPUTE_COMMAND in
        "xfx_q2")
            PID=$(gum input --placeholder "Enter Parton PID")
            INPUTS=$(gum input --placeholder "Enter inputs (e.g., '1e-5 100')")
            neopdf compute xfx_q2 "$PDF_NAME" -m "$MEMBER" --pid="$PID" $INPUTS
            ;;
        "alphas_q2")
            Q2=$(gum input --placeholder "Enter Q2 value")
            neopdf compute alphas_q2 "$PDF_NAME" -m "$MEMBER" -q "$Q2"
            ;;
        "xfx_q2_kt")
            PID=$(gum input --placeholder "Enter Parton PID")
            INPUTS=$(gum input --placeholder "Enter inputs (e.g., 'kt x q2')")
            neopdf compute xfx_q2_kt "$PDF_NAME" -m "$MEMBER" --pid="$PID" $INPUTS
            ;;
    esac
fi

# --- Write Subcommands ---
if [ "$COMMAND" == "write" ]; then
    WRITE_COMMAND=$(gum choose "convert" "combine-npdfs" "combine-alphas" "convert-tmd" "metadata")

    case $WRITE_COMMAND in
        "convert")
            PDF_NAME=$(gum input --placeholder "Enter the LHAPDF set name to convert")
            OUTPUT_PATH=$(gum input --placeholder "Enter the output file path for the NeoPDF file")
            neopdf write convert "$PDF_NAME" -o "$OUTPUT_PATH"
            ;;
        "combine-npdfs" | "combine-alphas")
            INPUT_METHOD=$(gum choose "direct" "file")

            if [ "$INPUT_METHOD" == "direct" ]; then
                PDF_NAMES=$(gum input --placeholder "Enter PDF set names (space-separated)")
                OUTPUT_PATH=$(gum input --placeholder "Enter the output file path for the combined NeoPDF file")
                neopdf write $WRITE_COMMAND -n $PDF_NAMES -o "$OUTPUT_PATH"
            else
                NAMES_FILE=$(gum input --placeholder "Enter the path to the file containing PDF set names")
                OUTPUT_PATH=$(gum input --placeholder "Enter the output file path for the combined NeoPDF file")
                neopdf write $WRITE_COMMAND -f "$NAMES_FILE" -o "$OUTPUT_PATH"
            fi
            ;;
        "convert-tmd")
            INPUT_PATH=$(gum input --placeholder "Enter the input configuration file path")
            OUTPUT_PATH=$(gum input --placeholder "Enter the output file path for the NeoPDF file")
            neopdf write convert-tmd -i "$INPUT_PATH" -o "$OUTPUT_PATH"
            ;;
        "metadata")
            FILE_PATH=$(gum input --placeholder "Enter the path to the NeoPDF file")
            KEY=$(gum input --placeholder "Enter the metadata key to update")
            VALUE=$(gum input --placeholder "Enter the new value for the key")
            neopdf write metadata --path "$FILE_PATH" --key "$KEY" --value "$VALUE"
            ;;
    esac
fi

# --- Install Subcommand ---
if [ "$COMMAND" == "install" ]; then
    PDF_NAME=$(gum input --placeholder "Enter PDF name")
    neopdf install "$PDF_NAME"
fi
