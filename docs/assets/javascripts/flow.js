// Renders the flow diagrams of the documentation with React Flow.
//
// A diagram is declared in Markdown as a JSON spec inside a `.neopdf-flow` block:
//
//   <div class="neopdf-flow"><script type="application/json">
//   {"nodes": [{"id": "a", "color": "orange", "title": "...", "detail": "...", "row": 0, "col": 0}],
//    "edges": [["a", "b", "optional label"]]}
//   </script></div>
//
// Nodes are laid out top to bottom on a grid of rows and (possibly fractional) columns, and
// `color` names one of the palette colours of `.neopdf-flow` in custom.css. React Flow is
// only downloaded on pages that contain a diagram.

const ESM = "https://esm.sh";
const REACT = `${ESM}/react@19.3.0`;
const REACT_DOM = `${ESM}/react-dom@19.3.0/client?deps=react@19.3.0`;
const XYFLOW = `${ESM}/@xyflow/react@12.12.0?deps=react@19.3.0,react-dom@19.3.0`;
const XYFLOW_CSS = `${ESM}/@xyflow/react@12.12.0/dist/style.css`;
const HTM = `${ESM}/htm@3.1.1`;

// Grid geometry in React Flow pixels; the node width is shared with custom.css.
const NODE_WIDTH = 230;
const COL_STEP = NODE_WIDTH + 50;
const ROW_STEP = 115;

async function render(containers) {
  const link = Object.assign(document.createElement("link"), { rel: "stylesheet", href: XYFLOW_CSS });
  document.head.append(link);

  const [{ default: React }, { createRoot }, xyflow, { default: htm }] = await Promise.all(
    [REACT, REACT_DOM, XYFLOW, HTM].map((url) => import(url)),
  );
  const { Handle, MarkerType, Position, ReactFlow } = xyflow;
  const html = htm.bind(React.createElement);

  const StepNode = ({ data }) => html`
    <div className=${`neopdf-flow__node neopdf-flow__node--${data.color}`}>
      <${Handle} type="target" position=${Position.Top} isConnectable=${false} />
      <div className="neopdf-flow__title">${data.title}</div>
      <div className="neopdf-flow__detail">${data.detail}</div>
      <${Handle} type="source" position=${Position.Bottom} isConnectable=${false} />
    </div>
  `;
  const nodeTypes = { step: StepNode };

  for (const container of containers) {
    const spec = JSON.parse(container.querySelector("script").textContent);
    const colors = Object.fromEntries(spec.nodes.map((node) => [node.id, node.color]));
    const nodes = spec.nodes.map(({ id, row, col = 0, ...data }) => ({
      id,
      type: "step",
      data,
      position: { x: col * COL_STEP, y: row * ROW_STEP },
    }));
    // Each edge carries the colour of the node it leaves, so the data visibly flows down.
    const edges = spec.edges.map(([source, target, label]) => ({
      id: `${source}->${target}`,
      source,
      target,
      label,
      type: "smoothstep",
      animated: true,
      className: `neopdf-flow__edge--${colors[source]}`,
      markerEnd: { type: MarkerType.ArrowClosed, color: `var(--neopdf-flow-${colors[source]})` },
    }));
    const rows = Math.max(...spec.nodes.map((node) => node.row)) + 1;
    container.style.setProperty("--neopdf-flow-rows", rows);

    createRoot(container).render(html`
      <${ReactFlow}
        nodes=${nodes}
        edges=${edges}
        nodeTypes=${nodeTypes}
        colorMode="dark"
        fitView
        fitViewOptions=${{ padding: 0.06 }}
        nodesDraggable=${false}
        nodesConnectable=${false}
        elementsSelectable=${false}
        panOnDrag=${false}
        zoomOnScroll=${false}
        zoomOnPinch=${false}
        zoomOnDoubleClick=${false}
        preventScrolling=${false}
      />
    `);
  }
}

const containers = document.querySelectorAll(".neopdf-flow");
if (containers.length > 0) render(containers);
