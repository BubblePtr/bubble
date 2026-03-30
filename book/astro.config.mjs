import { defineConfig } from "astro/config";
import react from "@astrojs/react";
import starlight from "@astrojs/starlight";

export default defineConfig({
  integrations: [
    react(),
    starlight({
      title: "Bubble Book",
      social: [],
      sidebar: [
        {
          label: "Introduction",
          items: [
            { label: "Overview", slug: "index" },
            { label: "Chapter 1: REPL", slug: "ch01-repl" }
          ]
        }
      ]
    })
  ]
});
