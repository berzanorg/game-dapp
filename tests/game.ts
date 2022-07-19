import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Game } from "../target/types/game";

describe("game", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);


  const program = anchor.workspace.Game as Program<Game>;


  it("Can set and change name!", async () => {
    const [userStatsPDA, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('user-stats'),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )

    await program.methods
      .createUserStats('Berzan')
      .accounts({
        user: provider.wallet.publicKey,
        userStats: userStatsPDA,
      })
      .rpc()

    expect((await program.account.userStats.fetch(userStatsPDA)).name).
      to.equal('Berzan')
    

      
    await program.methods
      .changeUserName('Zan')
      .accounts({
        user: provider.wallet.publicKey,
        userStats: userStatsPDA,
      })
      .rpc()

    expect((await program.account.userStats.fetch(userStatsPDA)).name).
    to.equal('Zan')
    
  })
})
