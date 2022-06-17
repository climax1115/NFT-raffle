import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Raffle, IDL } from '../target/types/raffle';
const { 
  SystemProgram, 
  Keypair, 
  PublicKey, 
  LAMPORTS_PER_SOL, 
  clusterApiUrl, 
  SYSVAR_RENT_PUBKEY,
  SYSVAR_CLOCK_PUBKEY,
 } = anchor.web3;
 import {
  AccountLayout,
  TOKEN_PROGRAM_ID,
  createAccount,
  createMint,
  getMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  mintTo,
  createInitializeAccountInstruction,
  createAssociatedTokenAccount,
  getAssociatedTokenAddress
} from "@solana/spl-token";
import * as utils from './utils';

describe('raffle', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Raffle as Program<Raffle>;
  const provider = anchor.AnchorProvider.env();

  const lotteryKey = Keypair.generate().publicKey;
  const lotteryKey2 = Keypair.generate().publicKey;
  const vault = Keypair.generate();
  it('Create Sol Raffle', async () => {
    const [lottery, bump] = await PublicKey.findProgramAddress([
      Buffer.from("lottery"), 
      provider.wallet.payer.publicKey.toBuffer(),
      lotteryKey.toBuffer(),
    ], program.programId)

    const now = new Date();
    now.setDate(now.getDate() + 1);
    const startDate = parseInt(now.getTime() / 1000);
    const endDate = startDate + 3 * 24 * 3600;

    const tx = await program.methods.createSolLottery(
      bump, 
      lotteryKey,
      new anchor.BN(startDate), 
      new anchor.BN(endDate), 
      new anchor.BN(0.1 * LAMPORTS_PER_SOL),
      new anchor.BN(2), 
      new anchor.BN(1), 
      new anchor.BN(1)).accounts({
        lottery,
        creator: provider.wallet.payer.publicKey,
        vault: vault.publicKey,
        rentSysvar: SYSVAR_RENT_PUBKEY,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
        systemProgram: SystemProgram.programId,
      }).rpc();

    const lotteryAccount = await program.account.lottery.fetch(lottery);
    console.log(lotteryAccount)
    // console.log("Your transaction signature", tx);
  });

  it('Update Sol Raffle', async () => {
    const [lottery] = await PublicKey.findProgramAddress([
      Buffer.from("lottery"), 
      provider.wallet.payer.publicKey.toBuffer(),
      lotteryKey.toBuffer(),
    ], program.programId)

    const now = new Date();
    now.setDate(now.getDate() + 2);
    const startDate = parseInt(now.getTime() / 1000);
    const endDate = startDate + 4 * 24 * 3600;

    const tx = await program.methods.updateLottery(
      new anchor.BN(startDate), 
      new anchor.BN(endDate), 
      new anchor.BN(0.1 * LAMPORTS_PER_SOL),
      new anchor.BN(3), 
      new anchor.BN(1), 
      new anchor.BN(1)).accounts({
        lottery,
        creator: provider.wallet.payer.publicKey,
        rentSysvar: SYSVAR_RENT_PUBKEY,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
        systemProgram: SystemProgram.programId,
      }).rpc();

    const lotteryAccount = await program.account.lottery.fetch(lottery);
    console.log(lotteryAccount)
    // console.log("Your transaction signature", tx);
  });

  const user = Keypair.generate();
  it('Create discount', async () => {
    const [lottery] = await PublicKey.findProgramAddress([
      Buffer.from("lottery"), 
      provider.wallet.payer.publicKey.toBuffer(),
      lotteryKey.toBuffer(),
    ], program.programId)

    const [discount, bump] = await PublicKey.findProgramAddress([
      Buffer.from("discount"), 
      lottery.toBuffer(),
      provider.wallet.payer.publicKey.toBuffer(),
    ], program.programId)

    const tx = await program.methods.createDiscount(
      bump, 
      1,
      user.publicKey,
      30).accounts({
        lottery,
        discount,
        creator: provider.wallet.payer.publicKey,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
        systemProgram: SystemProgram.programId,
      }).rpc();

    const discountAccount = await program.account.discount.fetch(discount);
    console.log(discountAccount.discount);
  })

  it('Update discount', async () => {
    const [lottery] = await PublicKey.findProgramAddress([
      Buffer.from("lottery"), 
      provider.wallet.payer.publicKey.toBuffer(),
      lotteryKey.toBuffer(),
    ], program.programId)
    
    const [discount] = await PublicKey.findProgramAddress([
      Buffer.from("discount"), 
      lottery.toBuffer(),
      provider.wallet.payer.publicKey.toBuffer(),
    ], program.programId)

    const tx = await program.methods.updateDiscount(50).accounts({
        lottery,
        discount,
        creator: provider.wallet.payer.publicKey,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
        systemProgram: SystemProgram.programId,
      }).rpc();

    const discountAccount = await program.account.discount.fetch(discount);
    console.log(discountAccount.discount)
  })

  it ('Buy ticket', async () => {
    const userWallet = new anchor.Wallet(user);
    const userProvider = new anchor.AnchorProvider(provider.connection, userWallet, anchor.AnchorProvider.defaultOptions());
    const userProgram = new anchor.Program(IDL, program.programId, userProvider);

    await utils.sendLamports(provider, user.publicKey, LAMPORTS_PER_SOL);

    const [lottery] = await PublicKey.findProgramAddress([
      Buffer.from("lottery"), 
      provider.wallet.payer.publicKey.toBuffer(),
      lotteryKey.toBuffer(),
    ], program.programId)

    const [discount] = await PublicKey.findProgramAddress([
      Buffer.from("discount"), 
      lottery.toBuffer(),
      provider.wallet.payer.publicKey.toBuffer(),
    ], program.programId)
    const discountAccount = await provider.connection.getAccountInfo(discount);
    let rDiscount = lottery;
    if (discountAccount !== null) {
      rDiscount = discount;
    }

    const [ticket, bump] = await PublicKey.findProgramAddress([
      Buffer.from("ticket"), 
      lottery.toBuffer(),
      user.publicKey.toBuffer(),
    ], program.programId)

    const tx = await userProgram.methods.createTicket(bump).accounts({
      lottery,
      ticket,
      buyer: user.publicKey,
      systemProgram: SystemProgram.programId,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
    }).rpc();

    const tx2 = await userProgram.methods.buyTicketWithSol(1).accounts({
      lottery,
      ticket,
      buyer: user.publicKey,
      vault: vault.publicKey,
      discount: rDiscount,
      systemProgram: SystemProgram.programId,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
    }).rpc();

    const ticketAccount = await userProgram.account.ticket.fetch(ticket);
    console.log(ticketAccount)
    console.log("Vault balance: ", await provider.connection.getBalance(vault.publicKey));
  })


  it ('Buy ticket2 and Fail limited buy', async () => {
    const user = Keypair.generate();
    const userWallet = new anchor.Wallet(user);
    const userProvider = new anchor.AnchorProvider(provider.connection, userWallet, anchor.AnchorProvider.defaultOptions());
    const userProgram = new anchor.Program(IDL, program.programId, userProvider);

    await utils.sendLamports(provider, user.publicKey, LAMPORTS_PER_SOL);

    const [lottery] = await PublicKey.findProgramAddress([
      Buffer.from("lottery"), 
      provider.wallet.payer.publicKey.toBuffer(),
      lotteryKey.toBuffer(),
    ], program.programId)

    const [discount] = await PublicKey.findProgramAddress([
      Buffer.from("discount"), 
      lottery.toBuffer(),
      provider.wallet.payer.publicKey.toBuffer(),
    ], program.programId)
    const discountAccount = await provider.connection.getAccountInfo(discount);
    let rDiscount = lottery;
    if (discountAccount !== null) {
      rDiscount = discount;
    }

    const [ticket, bump] = await PublicKey.findProgramAddress([
      Buffer.from("ticket"), 
      lottery.toBuffer(),
      user.publicKey.toBuffer(),
    ], program.programId)

    const tx = await userProgram.methods.createTicket(bump).accounts({
      lottery,
      ticket,
      buyer: user.publicKey,
      systemProgram: SystemProgram.programId,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
    }).rpc();

    const tx2 = await userProgram.methods.buyTicketWithSol(1).accounts({
      lottery,
      ticket,
      buyer: user.publicKey,
      discount: rDiscount,
      vault: vault.publicKey,
      systemProgram: SystemProgram.programId,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
    }).rpc();

    try {

    const tx3 = await userProgram.methods.buyTicketWithSol(1).accounts({
      lottery,
      ticket,
      buyer: user.publicKey,
      vault: vault.publicKey,
      systemProgram: SystemProgram.programId,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
    }).rpc();

    } catch(e) {
      console.log("Worked ticket limit per user");
    }

    const ticketAccount = await userProgram.account.ticket.fetch(ticket);
    console.log(ticketAccount)
    console.log("Vault balance: ", await provider.connection.getBalance(vault.publicKey));
  })

  it ('Fail Buy limited', async () => {
    try {
      const user = Keypair.generate();
      const userWallet = new anchor.Wallet(user);
      const userProvider = new anchor.AnchorProvider(provider.connection, userWallet, anchor.AnchorProvider.defaultOptions());
      const userProgram = new anchor.Program(IDL, program.programId, userProvider);

      await utils.sendLamports(provider, user.publicKey, LAMPORTS_PER_SOL);

      const [lottery] = await PublicKey.findProgramAddress([
        Buffer.from("lottery"), 
        provider.wallet.payer.publicKey.toBuffer(),
        lotteryKey.toBuffer(),
      ], program.programId)

      const [ticket, bump] = await PublicKey.findProgramAddress([
        Buffer.from("ticket"), 
        lottery.toBuffer(),
        user.publicKey.toBuffer(),
      ], program.programId)

      const tx = await userProgram.methods.createTicket(bump).accounts({
        lottery,
        ticket,
        buyer: user.publicKey,
        systemProgram: SystemProgram.programId,
          clockSysvar: SYSVAR_CLOCK_PUBKEY,
      }).rpc();

      const tx2 = await userProgram.methods.buyTicketWithSol(1).accounts({
        lottery,
        ticket,
        buyer: user.publicKey,
        vault: vault.publicKey,
        systemProgram: SystemProgram.programId,
          clockSysvar: SYSVAR_CLOCK_PUBKEY,
      }).rpc();
    } catch(e) {
      console.log("worked ticket limited");
    }

  })

  it ('Close lottery', async () => {
    const [lottery] = await PublicKey.findProgramAddress([
      Buffer.from("lottery"), 
      provider.wallet.payer.publicKey.toBuffer(),
      lotteryKey.toBuffer(),
    ], program.programId)

    const tx = await program.methods.closeLottery([
      provider.wallet.payer.publicKey, 
      PublicKey.default,
      PublicKey.default,
      PublicKey.default,
      PublicKey.default,
      PublicKey.default,
      PublicKey.default,
      PublicKey.default,
      PublicKey.default,
      PublicKey.default,
      PublicKey.default,
    ]).accounts({
      lottery,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
    }).rpc();

    const lotteryAccount = await program.account.lottery.fetch(lottery);
    console.log(lotteryAccount)
  })

  let splToken: PublicKey = null;
  it ('Mint token', async () => {
    splToken = await utils.createMint(provider, 9);
  })
  it('Create Spl Raffle', async () => {
    const [lottery, bump] = await PublicKey.findProgramAddress([
      Buffer.from("lottery"), 
      provider.wallet.payer.publicKey.toBuffer(),
      lotteryKey2.toBuffer(),
    ], program.programId)


    const startDate = parseInt((new Date()).getTime() / 1000);
    const endDate = startDate + 3 * 24 * 3600;
    const ataAccount = await createAssociatedTokenAccount(provider.connection, provider.wallet.payer, splToken, provider.wallet.payer.publicKey);

    const tx = await program.methods.createSplLottery(
      bump, 
      lotteryKey2,
      new anchor.BN(startDate), 
      new anchor.BN(endDate), 
      new anchor.BN(0.1 * LAMPORTS_PER_SOL),
      new anchor.BN(2), 
      new anchor.BN(1), 
      new anchor.BN(1)).accounts({
        lottery,
        creator: provider.wallet.payer.publicKey,
        mint: splToken,
        vault: ataAccount,
        rentSysvar: SYSVAR_RENT_PUBKEY,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      }).rpc();

    const lotteryAccount = await program.account.lottery.fetch(lottery);
    console.log(lotteryAccount)
    // console.log("Your transaction signature", tx);
  });

  it ('Buy spl ticket', async () => {
    const user = Keypair.generate();
    const userWallet = new anchor.Wallet(user);
    const userProvider = new anchor.AnchorProvider(provider.connection, userWallet, anchor.AnchorProvider.defaultOptions());
    const userProgram = new anchor.Program(IDL, program.programId, userProvider);

    await utils.sendLamports(provider, user.publicKey, LAMPORTS_PER_SOL);
    const ataAccount = await createAssociatedTokenAccount(provider.connection, user, splToken, user.publicKey);

    const [lottery] = await PublicKey.findProgramAddress([
      Buffer.from("lottery"), 
      provider.wallet.payer.publicKey.toBuffer(),
      lotteryKey2.toBuffer(),
    ], program.programId)

    const [discount] = await PublicKey.findProgramAddress([
      Buffer.from("discount"), 
      lottery.toBuffer(),
      provider.wallet.payer.publicKey.toBuffer(),
    ], program.programId)
    const discountAccount = await provider.connection.getAccountInfo(discount);
    let rDiscount = lottery;
    if (discountAccount !== null) {
      rDiscount = discount;
    }

    const [ticket, bump] = await PublicKey.findProgramAddress([
      Buffer.from("ticket"), 
      lottery.toBuffer(),
      user.publicKey.toBuffer(),
    ], program.programId)

    const tx = await userProgram.methods.createTicket(bump).accounts({
      lottery,
      ticket,
      buyer: user.publicKey,
      systemProgram: SystemProgram.programId,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
    }).rpc();

    const vault = await getAssociatedTokenAddress(splToken, provider.wallet.payer.publicKey);
    await utils.mintToAccount(provider, splToken, ataAccount, LAMPORTS_PER_SOL);
    console.log("ata balance: ", await provider.connection.getTokenAccountBalance(ataAccount));

    const tx2 = await userProgram.methods.buyTicketWithSpl(1).accounts({
      lottery,
      ticket,
      buyer: user.publicKey,
      vault,
      discount: rDiscount,
      buyerTokenAccount: ataAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
    }).rpc();

    const ticketAccount = await userProgram.account.ticket.fetch(ticket);
    console.log(ticketAccount)
    console.log("Vault balance: ", await provider.connection.getTokenAccountBalance(vault));
  })

  it ('Filter lottery and ticket', async () => {
    const lotteries = await program.account.lottery.all({
            memcmp: {
                offset: 9,
                bytes: provider.wallet.payer.publicKey.toBase58(),
            },
        });
    console.log(lotteries)
    const pubkeys = lotteries.map(l => l.publicKey);
    const accountInfo = await provider.connection.getAccountInfo(pubkeys[0]);
    
  })

});
