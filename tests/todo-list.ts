import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoList } from "../target/types/todo_list";

describe("todo-list", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.TodoList as Program<TodoList>;

    it("Is initialized!", async () => {
        // Add your test here.
        const noteAccount = anchor.web3.Keypair.generate();
        const noteWriter = (program.provider as anchor.AnchorProvider).wallet;

        await program.methods.initialize()
            .accounts({
                todoNote: noteAccount.publicKey,
                noteWriter: noteWriter.publicKey,
            })
            .signers([noteAccount])
            .rpc();
 
        let createdNote = await program.account.todoNote.fetch(noteAccount.publicKey);
        console.log(createdNote);

        await program.methods.addTodo('do laundary')
            .accounts({
                todoNote: noteAccount.publicKey,
                writer: noteWriter.publicKey,
            })
            .signers([])
            .rpc();

        createdNote = await program.account.todoNote.fetch(noteAccount.publicKey);
        console.log(createdNote);

        //await program.methods.addTodo('something greater than 50byte. something greater than 50byte')
        //    .accounts({
        //        todoNote: noteAccount.publicKey,
        //    })
        //    .signers([])
        //    .rpc();
        
        await program.methods.updateTodo(
                { task: 'do laundary', isFinished: false },
                { task: 'do laundary2', isFinished: false },
            ).accounts({
                todoNote: noteAccount.publicKey,
            })
            .signers([])
            .rpc();

        createdNote = await program.account.todoNote.fetch(noteAccount.publicKey);
        console.log(createdNote);

        await program.methods.removeTodo('do laundary2')
            .accounts({ todoNote: noteAccount.publicKey })
            .signers([])
            .rpc();

        createdNote = await program.account.todoNote.fetch(noteAccount.publicKey);
        console.log(createdNote);
    });
});
