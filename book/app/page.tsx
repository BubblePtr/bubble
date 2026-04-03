import Link from "next/link";
import styles from "./page.module.css";

const capabilityModules = [
  {
    icon: "ri-hammer-line",
    title: "Tool Calling"
  },
  {
    icon: "ri-chat-smile-ai-line",
    title: "Context Engineering"
  },
  {
    icon: "ri-brain-line",
    title: "Memory Systems"
  },
  {
    icon: "ri-remix-line",
    title: "Evaluation Harness"
  },
  {
    icon: "ri-loop-right-line",
    title: "Self-Improvement"
  }
];

function ArrowIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true" className={styles.arrowIcon}>
      <path
        d="M7 12h10m0 0-4-4m4 4-4 4"
        fill="none"
        stroke="currentColor"
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth="1.8"
      />
    </svg>
  );
}

function StarRow() {
  return (
    <div className={styles.stars} aria-hidden="true">
      {Array.from({ length: 5 }).map((_, index) => (
        <svg key={index} viewBox="0 0 24 24" className={styles.star}>
          <path
            d="m12 2.9 2.78 5.63 6.22.91-4.5 4.39 1.06 6.2L12 17.14 6.44 20.03l1.06-6.2L3 9.44l6.22-.91L12 2.9Z"
            fill="currentColor"
          />
        </svg>
      ))}
    </div>
  );
}

export default function HomePage() {
  return (
    <main className={styles.page}>
      <div className={styles.glowLayer} aria-hidden="true">
        <span className={styles.glowOne} />
        <span className={styles.glowTwo} />
        <span className={styles.glowThree} />
      </div>

      <div className={styles.shell}>
        <header className={styles.navWrap}>
          <div className={styles.navbar}>
            <Link href="/" className={styles.brand}>
              Learn Agent Build
            </Link>

            <nav className={styles.navLinks} aria-label="Primary">
              <a href="#overview">Overview</a>
              <a href="#modules">Modules</a>
              <Link href="/docs">Read</Link>
            </nav>

            <Link href="/docs" className={styles.navCta}>
              Start Reading
              <span className={styles.navCtaIcon}>
                <ArrowIcon />
              </span>
            </Link>
          </div>
        </header>

        <section className={styles.hero} id="overview">
          <div className={styles.copyColumn}>
            <div className={styles.proofBadge}>
              <StarRow />
              <span>Focused notes for builders shipping real agent systems</span>
            </div>

            <p className={styles.eyebrow}>Learn Agent Build</p>
            <h1 className={styles.headline}>Build agent systems with clarity</h1>
            <p className={styles.subheadline}>
              一本讲清楚 agent 架构拆分、context engineering、长期记忆与可观测性的实战型网站书。
              不卖概念，直接解释系统为什么这样设计、应该先做什么、哪些复杂度值得引入。
            </p>

            <div className={styles.actions}>
              <Link href="/docs" className={styles.primaryCta}>
                <span>Start Reading</span>
                <span className={styles.primaryIcon}>
                  <ArrowIcon />
                </span>
              </Link>
              <a href="#modules" className={styles.secondaryLink}>
                See the modules
              </a>
            </div>
          </div>

          <div className={styles.visualColumn} aria-hidden="true">
            <div className={styles.orbFrame}>
              <video
                autoPlay
                loop
                muted
                playsInline
                className={styles.orbVideo}
                src="https://future.co/images/homepage/glassy-orb/orb-purple.webm"
              />
            </div>
          </div>
        </section>

        <section className={styles.capabilitySection} id="modules" aria-label="Core chapters">
          <p className={styles.logoTitle}>Core chapter modules</p>
          <div className={styles.capabilityRow}>
            {capabilityModules.map((item) => (
              <article key={item.title} className={styles.capabilityItem}>
                <span className={styles.capabilityIndex} aria-hidden="true">
                  <i className={item.icon} />
                </span>
                <h3>{item.title}</h3>
              </article>
            ))}
          </div>
        </section>

        <footer className={styles.footer}>MIT {new Date().getFullYear()} © Learn Agent Build.</footer>
      </div>
    </main>
  );
}
