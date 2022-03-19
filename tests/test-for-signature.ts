const anchor = require('@project-serum/anchor');
const { BN } = require('bn.js');
const { SystemProgram } = anchor.web3;
const solanaWeb3 = require('@solana/web3.js');

describe('payment_channel', async () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);
  
  const messageAccountKeypair= anchor.web3.Keypair.generate();
  const treasuryWallet = anchor.web3.Keypair.generate();
  const program = anchor.workspace.PaymentChannel;
  const user1 = anchor.web3.Keypair.generate();
  const user1PaymentUser = anchor.web3.Keypair.generate();

  it('creates user1', async () => {
    const signature = await program.provider.connection.requestAirdrop(user1.publicKey, 2);
    await program.provider.connection.confirmTransaction(signature);
    await program.rpc.createUser("User1",{
      accounts: {
        paymentUser: user1PaymentUser.publicKey,
        user: user1.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [user1, user1PaymentUser]
    });
  });

  it('Creates message account', async () => {
    const signature = await program.provider.connection.requestAirdrop(treasuryWallet.publicKey, 2);
    await program.provider.connection.confirmTransaction(signature);
    
    await program.rpc.createMessageAccount("My message!!",{
      accounts: {
        multisigWallet: messageAccountKeypair.publicKey,
        owner: treasuryWallet.publicKey,
        user1: user1.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [user1, treasuryWallet, messageAccountKeypair]
    });

  });

  it('Updates wallet contributions', async () => {
    await program.rpc.updateMessage("New Message!!",{
      accounts: {
        multisigWallet: messageAccountKeypair.publicKey,
        user1: user1.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [user1]
    });
    const messageAccount = await program.account.messageAccount.fetch(messageAccountKeypair.publicKey);
    console.log(messageAccount);
  });
        
});