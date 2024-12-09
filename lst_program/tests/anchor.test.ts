// No imports needed: web3, anchor, pg and more are globally available
describe("LST Program Test", () => {
  it("initialize", async () => {
    // Generate a new keypair for the pool state account
    const poolStateKp = new web3.Keypair();

    // Admin's public key (use the wallet's public key for simplicity)
    const admin = pg.wallet.publicKey;

    // Fee basis points (example: 100 basis points = 1%)
    const feeBasisPoints = 100; // Use a regular number for u16

    // Send the initialize transaction
    const txHash = await pg.program.methods
      .initialize(admin, feeBasisPoints) // Pass feeBasisPoints as a number
      .accounts({
        poolState: poolStateKp.publicKey,
        admin: admin,
        adminFeeAccount: pg.wallet.publicKey, // Use the same wallet for the fee account in this test
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([poolStateKp])
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm the transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch the created pool state account
    const poolState = await pg.program.account.poolState.fetch(
      poolStateKp.publicKey
    );

    console.log("Pool state initialized:", {
      admin: poolState.admin.toString(),
      totalStaked: poolState.totalStaked.toString(),
      totalMinted: poolState.totalMinted.toString(),
      rewardsCompounded: poolState.rewardsCompounded.toString(),
      feeBasisPoints: poolState.feeBasisPoints.toString(),
      paused: poolState.paused,
    });

    // Assertions to ensure the pool state matches the expected values
    assert.strictEqual(poolState.admin.toString(), admin.toString());
    assert.strictEqual(poolState.totalStaked.toString(), "0");
    assert.strictEqual(poolState.totalMinted.toString(), "0");
    assert.strictEqual(poolState.rewardsCompounded.toString(), "0");
    assert.strictEqual(poolState.feeBasisPoints, feeBasisPoints);
    assert.strictEqual(poolState.paused, false);
  });
});
