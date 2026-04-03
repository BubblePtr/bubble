"use client";

import { useState } from "react";

// Remixicon class names (font CSS is loaded globally in layout.tsx)
const NODE_ICONS: Record<string, string> = {
  user: "ri-user-line",
  message: "ri-chat-1-line",
  assistant: "ri-robot-2-line",
  tool: "ri-wrench-line",
};

interface Step {
  active: string[];
  edges: string[];
  desc: string;
}

const STEPS: Step[] = [
  { active: ["user"], edges: [], desc: "用户输入 prompt" },
  {
    active: ["user", "message"],
    edges: ["user-message"],
    desc: "输入被包装成 Message 存入聊天记录",
  },
  {
    active: ["message", "assistant"],
    edges: ["message-assistant"],
    desc: "聊天记录发送给大模型（Assistant）",
  },
  {
    active: ["assistant", "tool"],
    edges: ["assistant-tool"],
    desc: "大模型决定调用 Tool 执行操作",
  },
  {
    active: ["tool", "message"],
    edges: ["tool-message"],
    desc: "Tool 的执行结果写回 Message",
  },
  {
    active: ["message", "assistant"],
    edges: ["message-assistant"],
    desc: "带上 Tool 结果，再次请求大模型",
  },
  {
    active: ["assistant", "message"],
    edges: ["assistant-message"],
    desc: "大模型给出最终回答，存入 Message",
  },
  {
    active: ["message", "user"],
    edges: ["message-user"],
    desc: "回答展示给用户，等待下一次输入",
  },
];

const NODE_W = 110;
const NODE_H = 68;

const NODE_SUB: Record<string, string> = {
  user: "用户",
  message: "聊天记录",
  assistant: "大模型",
  tool: "工具",
};

const NODES: Record<string, { x: number; y: number; label: string }> = {
  user: { x: 60, y: 40, label: "User" },
  message: { x: 270, y: 40, label: "Message" },
  assistant: { x: 270, y: 200, label: "Assistant" },
  tool: { x: 60, y: 200, label: "Tool" },
};

const EDGES: Record<string, { from: string; to: string }> = {
  "user-message": { from: "user", to: "message" },
  "message-assistant": { from: "message", to: "assistant" },
  "assistant-tool": { from: "assistant", to: "tool" },
  "tool-message": { from: "tool", to: "message" },
  "assistant-message": { from: "assistant", to: "message" },
  "message-user": { from: "message", to: "user" },
};

const COLORS = {
  active: "hsl(212, 100%, 50%)",
  inactive: "hsl(220, 10%, 75%)",
  bg: "hsl(220, 10%, 97%)",
  activeBg: "hsl(212, 100%, 95%)",
  text: "hsl(220, 20%, 20%)",
  dimText: "hsl(220, 10%, 55%)",
};

function nodeCenter(id: string) {
  const n = NODES[id];
  return { x: n.x + NODE_W / 2, y: n.y + NODE_H / 2 };
}

function edgeEndpoint(
  center: { x: number; y: number },
  target: { x: number; y: number },
) {
  const dx = target.x - center.x;
  const dy = target.y - center.y;
  const absDx = Math.abs(dx);
  const absDy = Math.abs(dy);
  const scaleX = (NODE_W / 2 + 2) / absDx;
  const scaleY = (NODE_H / 2 + 2) / absDy;
  const scale = Math.min(scaleX, scaleY);
  return { x: center.x + dx * scale, y: center.y + dy * scale };
}

function renderEdge(edgeId: string, isActive: boolean) {
  const edge = EDGES[edgeId];
  if (!edge) return null;
  const fromC = nodeCenter(edge.from);
  const toC = nodeCenter(edge.to);
  const p1 = edgeEndpoint(fromC, toC);
  const p2 = edgeEndpoint(toC, fromC);
  return (
    <line
      key={`${edgeId}-${isActive}`}
      x1={p1.x}
      y1={p1.y}
      x2={p2.x}
      y2={p2.y}
      stroke={isActive ? COLORS.active : COLORS.inactive}
      strokeWidth={isActive ? 2.5 : 1}
      strokeDasharray={isActive ? "none" : "6 4"}
      markerEnd={`url(#agent-ah-${isActive ? "on" : "off"})`}
      style={{ transition: "stroke 0.3s, stroke-width 0.3s" }}
    />
  );
}

export function AgentLoopAnimation() {
  const [step, setStep] = useState(-1);
  const current = step >= 0 ? STEPS[step] : null;

  const next = () => setStep((s) => (s + 1 >= STEPS.length ? -1 : s + 1));
  const prev = () => setStep((s) => (s <= 0 ? -1 : s - 1));
  const reset = () => setStep(-1);

  return (
    <div style={{ margin: "2rem 0" }}>
      <svg
        viewBox="0 0 440 310"
        width="100%"
        style={{ maxWidth: 500, display: "block", margin: "0 auto" }}
      >
        <defs>
          <marker
            id="agent-ah-on"
            markerWidth="8"
            markerHeight="6"
            refX="7"
            refY="3"
            orient="auto"
          >
            <polygon points="0 0, 8 3, 0 6" fill={COLORS.active} />
          </marker>
          <marker
            id="agent-ah-off"
            markerWidth="8"
            markerHeight="6"
            refX="7"
            refY="3"
            orient="auto"
          >
            <polygon points="0 0, 8 3, 0 6" fill={COLORS.inactive} />
          </marker>
        </defs>

        {/* Background edges */}
        {Object.keys(EDGES).map((id) => renderEdge(id, false))}

        {/* Active edges */}
        {current?.edges.map((id) => renderEdge(id, true))}

        {/* Nodes */}
        {Object.entries(NODES).map(([id, node]) => {
          const isActive = current?.active.includes(id) ?? false;
          const iconCls = NODE_ICONS[id];
          return (
            <g key={id}>
              <rect
                x={node.x}
                y={node.y}
                width={NODE_W}
                height={NODE_H}
                rx={10}
                fill={isActive ? COLORS.activeBg : COLORS.bg}
                stroke={isActive ? COLORS.active : COLORS.inactive}
                strokeWidth={isActive ? 2.5 : 1.5}
                style={{ transition: "all 0.3s" }}
              />
              <foreignObject
                x={node.x}
                y={node.y}
                width={NODE_W}
                height={NODE_H}
              >
                <div
                  style={{
                    width: "100%",
                    height: "100%",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    gap: 6,
                  }}
                >
                  <i
                    className={iconCls}
                    style={{
                      fontSize: 16,
                      lineHeight: 1,
                      color: isActive ? COLORS.active : COLORS.dimText,
                      transition: "color 0.3s",
                    }}
                  />
                  <span
                    style={{
                      fontSize: 13,
                      fontWeight: isActive ? 700 : 500,
                      color: isActive ? COLORS.active : COLORS.text,
                      transition: "color 0.3s",
                      userSelect: "none",
                    }}
                  >
                    {node.label}
                  </span>
                </div>
              </foreignObject>
            </g>
          );
        })}
      </svg>

      <p
        style={{
          textAlign: "center",
          minHeight: "1.5em",
          color: current ? COLORS.text : COLORS.dimText,
          fontWeight: 500,
          margin: "0.5rem 0",
          fontSize: 15,
        }}
      >
        {current
          ? `Step ${step + 1}/${STEPS.length}：${current.desc}`
          : "点击「下一步」开始演示 Agent 循环"}
      </p>

      <div style={{ display: "flex", justifyContent: "center", gap: 8 }}>
        <button onClick={prev} disabled={step <= 0} style={btnStyle(step <= 0)}>
          ← 上一步
        </button>
        <button onClick={next} style={btnStyle(false)}>
          {step === STEPS.length - 1 ? "↻ 重置" : "下一步 →"}
        </button>
        {step >= 0 && (
          <button onClick={reset} style={btnStyle(false)}>
            重置
          </button>
        )}
      </div>
    </div>
  );
}

function btnStyle(disabled: boolean): React.CSSProperties {
  return {
    padding: "6px 16px",
    borderRadius: 6,
    border: "1px solid hsl(220,10%,80%)",
    background: disabled ? "hsl(220,10%,93%)" : "white",
    cursor: disabled ? "not-allowed" : "pointer",
    fontSize: 14,
  };
}
