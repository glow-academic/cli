#!/usr/bin/env node
// Demo bridge: drives the Keycloak login form headlessly so a VHS-recorded
// `glow login` can complete its PKCE round-trip without a human at the
// browser. The terminal narrative is what VHS captures; this is the
// off-camera "human clicks Sign In" half. See tapes/authentication-login.tape.
//
// Usage: node demo-login-bridge.mjs "<authorize_url>"
//   The URL is the one `glow login` prints ("If the browser doesn't open,
//   visit: …") — i.e. http://localhost:8000/authorize?…&redirect_uri=
//   http://127.0.0.1:<port>/callback&state=… . Following it redirects to the
//   Keycloak "Sign in to GLOW" form; we fill it and let the redirect chain
//   land back on the CLI's localhost callback listener.
import { createRequire } from "module";

// Playwright lives in the client repo, not here (this repo is Rust).
const require = createRequire("/Users/ashoksaravanan/Coding/glow-academic-client/");
const { chromium } = require("playwright");

const url = process.argv[2];
if (!url) {
  console.error("demo-login-bridge: no authorize URL given");
  process.exit(1);
}
const USER = process.env.DEMO_KC_USER || "default-superadmin@university.edu";
const PASS = process.env.DEMO_KC_PASS || "GlowDemo123!";

const browser = await chromium.launch({ headless: true });
try {
  const page = await browser.newPage();
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 20_000 });

  // Keycloak "Sign in to GLOW" form. The custom glow-gradient-login theme
  // keeps #kc-form-login at display:none, so fill()/click() (which require
  // visibility) won't work — set the field values and submit the form
  // programmatically instead. The POST to login-actions/authenticate is what
  // matters; CSS visibility is irrelevant to it.
  await page.waitForSelector("#username", { state: "attached", timeout: 15_000 });
  await Promise.all([
    page
      .waitForURL(/127\.0\.0\.1.*callback|oidc-callback/i, { timeout: 20_000 })
      .catch(() => undefined),
    page.evaluate(
      ({ u, p }) => {
        document.querySelector("#username").value = u;
        document.querySelector("#password").value = p;
        document.querySelector("#kc-form-login").submit();
      },
      { u: USER, p: PASS },
    ),
  ]);
  await page.waitForTimeout(1500);
} finally {
  await browser.close();
}
