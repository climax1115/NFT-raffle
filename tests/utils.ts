const anchor = require("@project-serum/anchor");
const token = require("@solana/spl-token");

async function createMint(provider, decimals) {
    const mint = await token.createMint(
        provider.connection,
        provider.wallet.payer,
        provider.wallet.payer.publicKey,
        null,
        decimals,
    );
    return mint;
}

async function sendLamports(
    provider,
    destination,
    amount
) {
    const tx = new anchor.web3.Transaction();
    tx.add(
        anchor.web3.SystemProgram.transfer(
            { 
                fromPubkey: provider.wallet.publicKey, 
                lamports: amount, 
                toPubkey: destination
            }
        )
    );
    await provider.sendAndConfirm(tx);
}

async function mintToAccount(
    provider,
    mint,
    destination,
    amount
) {
    const tx = new anchor.web3.Transaction();
    tx.add(
      token.createMintToInstruction(
        mint,
        destination,
        provider.wallet.publicKey,
        amount,
        [],
      )
    );
    await provider.sendAndConfirm(tx);
}

module.exports = {
    mintToAccount,
    createMint,
    sendLamports,
};
