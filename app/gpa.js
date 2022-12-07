const anchor = require("@project-serum/anchor");
const { PublicKey } = require("@solana/web3.js");
const idl = require("../target/idl/social_identity.json");

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const connection = provider.connection;

const programId = new PublicKey("HmoyrnYPqW5Rq5bhQfT8CCcstwpKize2dUiFooziyipe");
const program = new anchor.Program(idl, programId, provider);

const main = async () => {

    console.log("All user accounts")
    let userAccounts = await program.account.userAccount.all();
    userAccounts.forEach((acc) => {
        acc.publicKey = acc.publicKey.toString();
        console.log(acc);
    });

    console.log("all info accounts")
    let infoAccounts = await program.account.typeInfoAccount.all();
    infoAccounts.forEach((acc) => {
        acc.publicKey = acc.publicKey.toString();
        console.log(acc);
    });

}
main()