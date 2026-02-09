// Playground automatically provides pg.wallet and pg.connection
// Program methods are handled via the Client tab form

async function main() {
  console.log("=== CertiChain Demo ===");
  console.log("Wallet Address:", pg.wallet.publicKey.toString());

  // Show wallet balance
  const balance = await pg.connection.getBalance(pg.wallet.publicKey);
  console.log("Balance:", balance / 1_000_000_000, "SOL");

  // Instructions for judges/demo
  console.log("\n--- Instructions ---");
  console.log("1. Go to the Client tab in Playground.");
  console.log("2. Fill in the fields: student_name, course, college.");
  console.log("   Example:");
  console.log("      student_name: Rishav Shrestha");
  console.log("      course: Blockchain Basics");
  console.log("      college: Himalaya Darshan College");
  console.log("3. Click 'Run' to issue the certificate.");
  console.log("4. Check console for the certificate account public key.");
  console.log("\nEach issuance creates a unique certificate account linked to this wallet.");
}

main();
