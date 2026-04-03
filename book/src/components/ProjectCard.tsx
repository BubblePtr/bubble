"use client";

import type { CSSProperties, ReactNode } from "react";

interface ProjectCardProps {
  title: string;
  description: string;
  href: string;
  logo?: string;
  logoAlt?: string;
  logoBg?: string;
  logoScale?: number;
}

const styles = {
  card: {
    display: "flex",
    alignItems: "center",
    gap: "1.25rem",
    padding: "1rem 1.25rem",
    borderRadius: "1rem",
    border: "1px solid #e5e7eb",
    background: "#fff",
    transition: "box-shadow 0.2s, border-color 0.2s",
    textDecoration: "none",
    color: "inherit",
  } satisfies CSSProperties,
  cardHover: {
    boxShadow: "0 4px 24px rgba(0,0,0,0.08)",
    borderColor: "#d1d5db",
  } satisfies CSSProperties,
  logo: {
    width: 64,
    height: 64,
    objectFit: "contain",
    flexShrink: 0,
  } satisfies CSSProperties,
  logoFrame: {
    width: 64,
    height: 64,
    borderRadius: "0.75rem",
    flexShrink: 0,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    overflow: "hidden",
  } satisfies CSSProperties,
  logoPlaceholder: {
    width: 64,
    height: 64,
    borderRadius: "0.75rem",
    flexShrink: 0,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    fontSize: "1.5rem",
    fontWeight: 700,
    color: "#fff",
  } satisfies CSSProperties,
  body: {
    flex: 1,
    minWidth: 0,
  } satisfies CSSProperties,
  title: {
    margin: 0,
    fontSize: "1rem",
    fontWeight: 600,
    color: "#0d1726",
  } satisfies CSSProperties,
  desc: {
    margin: "0.25rem 0 0",
    fontSize: "0.875rem",
    lineHeight: 1.5,
    color: "#6b7280",
  } satisfies CSSProperties,
  arrow: {
    flexShrink: 0,
    fontSize: "1.25rem",
    color: "#9ca3af",
    transition: "transform 0.2s, color 0.2s",
  } satisfies CSSProperties,
} as const;

export function ProjectCard({
  title,
  description,
  href,
  logo,
  logoAlt,
  logoBg = "#e0e7ff",
  logoScale = 1,
}: ProjectCardProps) {
  return (
    <a
      href={href}
      target="_blank"
      rel="noopener noreferrer"
      style={styles.card}
      onMouseEnter={(e) => {
        Object.assign(e.currentTarget.style, styles.cardHover);
        const arrow = e.currentTarget.querySelector<HTMLElement>("[data-arrow]");
        if (arrow) {
          arrow.style.transform = "translateX(3px)";
          arrow.style.color = "#6b7280";
        }
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.boxShadow = "";
        e.currentTarget.style.borderColor = "#e5e7eb";
        const arrow = e.currentTarget.querySelector<HTMLElement>("[data-arrow]");
        if (arrow) {
          arrow.style.transform = "";
          arrow.style.color = "#9ca3af";
        }
      }}
    >
      {logo ? (
        <span style={{ ...styles.logoFrame, background: logoBg }}>
          <img
            src={logo}
            alt={logoAlt || title}
            style={{ ...styles.logo, transform: `scale(${logoScale})` }}
          />
        </span>
      ) : (
        <span style={{ ...styles.logoPlaceholder, background: logoBg }}>
          {title[0]}
        </span>
      )}
      <div style={styles.body}>
        <p style={styles.title}>{title}</p>
        <p style={styles.desc}>{description}</p>
      </div>
      <span data-arrow style={styles.arrow}>
        &#8250;
      </span>
    </a>
  );
}

export function ProjectCardGrid({ children }: { children: ReactNode }) {
  return (
    <div
      style={{
        display: "grid",
        gridTemplateColumns: "repeat(auto-fill, minmax(min(100%, 400px), 1fr))",
        gap: "0.75rem",
        marginTop: "1rem",
      }}
    >
      {children}
    </div>
  );
}
