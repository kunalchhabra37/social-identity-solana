const anchor = require("@project-serum/anchor");
const { PublicKey, SystemProgram } = require("@solana/web3.js");

const idl = require("../target/idl/social_identity.json");

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const connection = provider.connection;

const program_id = new PublicKey("HmoyrnYPqW5Rq5bhQfT8CCcstwpKize2dUiFooziyipe");
const program = new anchor.Program(idl, program_id, provider);

const main = async () => {
    const [userAccountPDA, _] = PublicKey.findProgramAddressSync(
        [
            anchor.utils.bytes.utf8.encode("user-account"),
            provider.wallet.publicKey.toBuffer()
        ],
        program_id
    );

    const trx = await program.methods.initialize("kunal", "i am a developer").accounts({
        userAccount: userAccountPDA,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
    }).signers([]).rpc();

    console.log("Your Account: ", userAccountPDA.toString(), "\n Trx: ", trx);
}
main()