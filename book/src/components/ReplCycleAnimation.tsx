"use client";

import { useState, useEffect } from "react";

const STEPS = [
  { id: "read", label: "Read", desc: "读取用户输入" },
  { id: "eval", label: "Eval", desc: "发送给大模型处理" },
  { id: "print", label: "Print", desc: "输出模型的回答" },
  { id: "loop", label: "Loop", desc: "回到等待输入状态" },
];

// Center positions for each node (circle center)
const CX = 200;
const CY = 160;
const R = 110; // orbit radius
const NODE_W = 80;
const NODE_H = 40;

// Top, Right, Bottom, Left
const ANGLES = [-Math.PI / 2, 0, Math.PI / 2, Math.PI];
const NODE_CENTERS = ANGLES.map((a) => ({
  x: CX + R * Math.cos(a),
  y: CY + R * Math.sin(a),
}));

const COLORS = {
  active: "hsl(212, 100%, 50%)",
  inactive: "hsl(220, 10%, 75%)",
  bg: "hsl(220, 10%, 97%)",
  activeBg: "hsl(212, 100%, 95%)",
  text: "hsl(220, 20%, 20%)",
  dimText: "hsl(220, 10%, 55%)",
};

function arcPath(from: { x: number; y: number }, to: { x: number; y: number }) {
  const mx = (from.x + to.x) / 2;
  const my = (from.y + to.y) / 2;
  // Offset control point outward from the center
  const dx = to.x - from.x;
  const dy = to.y - from.y;
  const len = Math.sqrt(dx * dx + dy * dy);
  const nx = -dy / len;
  const ny = dx / len;
  const bulge = 30;
  const cx = mx + nx * bulge;
  const cy = my + ny * bulge;
  return `M ${from.x} ${from.y} Q ${cx} ${cy} ${to.x} ${to.y}`;
}

function edgePoint(center: { x: number; y: number }, target: { x: number; y: number }, half: number) {
  const dx = target.x - center.x;
  const dy = target.y - center.y;
  const len = Math.sqrt(dx * dx + dy * dy);
  return {
    x: center.x + (dx / len) * half,
    y: center.y + (dy / len) * half,
  };
}

export function ReplCycleAnimation() {
  const [step, setStep] = useState(-1);
  const [isPlaying, setIsPlaying] = useState(false);

  useEffect(() => {
    if (!isPlaying) return;
    const timer = setInterval(() => {
      setStep((s) => {
        const next = s + 1;
        if (next >= STEPS.length) {
          setIsPlaying(false);
          return -1;
        }
        return next;
      });
    }, 1200);
    return () => clearInterval(timer);
  }, [isPlaying]);

  const handlePlay = () => {
    setStep(0);
    setIsPlaying(true);
  };

  const handleStep = () => {
    setStep((s) => (s + 1) % STEPS.length);
  };

  return (
    <div style={{ margin: "2rem 0" }}>
      <svg viewBox="0 0 400 320" width="100%" style={{ maxWidth: 460, display: "block", margin: "0 auto" }}>
        <defs>
          <marker id="repl-ah-on" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto">
            <polygon points="0 0, 8 3, 0 6" fill={COLORS.active} />
          </marker>
          <marker id="repl-ah-off" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto">
            <polygon points="0 0, 8 3, 0 6" fill={COLORS.inactive} />
          </marker>
        </defs>

        {/* Curved arrows between nodes */}
        {NODE_CENTERS.map((from, i) => {
          const to = NODE_CENTERS[(i + 1) % NODE_CENTERS.length];
          const isActive = step === i;
          const p1 = edgePoint(from, to, NODE_W / 2 + 4);
          const p2 = edgePoint(to, from, NODE_W / 2 + 4);
          return (
            <path
              key={`arrow-${i}`}
              d={arcPath(p1, p2)}
              fill="none"
              stroke={isActive ? COLORS.active : COLORS.inactive}
              strokeWidth={isActive ? 2.5 : 1.5}
              markerEnd={`url(#repl-ah-${isActive ? "on" : "off"})`}
              style={{ transition: "stroke 0.3s, stroke-width 0.3s" }}
            />
          );
        })}

        {/* Nodes */}
        {NODE_CENTERS.map((pos, i) => {
          const isActive = step === i;
          return (
            <g key={STEPS[i].id} style={{ cursor: "pointer" }} onClick={() => setStep(i)}>
              <rect
                x={pos.x - NODE_W / 2} y={pos.y - NODE_H / 2}
                width={NODE_W} height={NODE_H} rx={8}
                fill={isActive ? COLORS.activeBg : COLORS.bg}
                stroke={isActive ? COLORS.active : COLORS.inactive}
                strokeWidth={isActive ? 2.5 : 1.5}
                style={{ transition: "all 0.3s" }}
              />
              <text
                x={pos.x} y={pos.y + 5}
                textAnchor="middle"
                fontSize={15}
                fontWeight={isActive ? 700 : 500}
                fill={isActive ? COLORS.active : COLORS.text}
                style={{ transition: "fill 0.3s", userSelect: "none" }}
              >
                {STEPS[i].label}
              </text>
            </g>
          );
        })}
      </svg>

      <p style={{
        textAlign: "center",
        minHeight: "1.5em",
        color: step >= 0 ? COLORS.text : COLORS.dimText,
        fontWeight: 500,
        margin: "0.5rem 0",
      }}>
        {step >= 0 ? STEPS[step].desc : "点击下方按钮开始"}
      </p>

      <div style={{ display: "flex", justifyContent: "center", gap: 8 }}>
        <button
          onClick={handlePlay}
          disabled={isPlaying}
          style={btnStyle(isPlaying)}
        >
          ▶ 自动播放
        </button>
        <button
          onClick={handleStep}
          disabled={isPlaying}
          style={btnStyle(isPlaying)}
        >
          下一步 →
        </button>
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
