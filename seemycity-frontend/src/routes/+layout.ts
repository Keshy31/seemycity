// The app ships as a static SPA (adapter-static with an index.html fallback):
// the map needs the browser, detail routes are fully dynamic, and data comes
// from the Rust API — there is nothing useful to server-render at build time.
export const ssr = false;
