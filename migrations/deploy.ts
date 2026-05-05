// Migrations are an early feature. You may want to make
// sure they are idempotent before deploying to production.

import * as anchor from "@coral-xyz/anchor";

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);
  // Add deployment logic here if needed
};
