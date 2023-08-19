use anchor_lang::prelude::*;

declare_id!("7bfQJKPLfXtzwTN5TxLmoAP6jwEswHDQMPAy3hTBA4F2");

#[program]
pub mod todo_list {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.todo_note.writer = ctx.accounts.note_writer.key();
        Ok(())
    }

    pub fn add_todo(ctx: Context<TodoInstruction>, task: String) -> Result<()> {
        let todo_note = &mut ctx.accounts.todo_note;
        todo_note.add_todo(task)
    }

    pub fn remove_todo(ctx: Context<TodoInstruction>, task: String) -> Result<()> {
        let todo_note = &mut ctx.accounts.todo_note;
        todo_note.remove_todo(task)
    }

    pub fn update_todo(ctx: Context<TodoInstruction>, from: Todo, to: Todo) -> Result<()> {
        let todo_note = &mut ctx.accounts.todo_note;
        todo_note.update_todo(from, to)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = note_writer, space = 8 + TodoNote::MAX_SIZE)]
    pub todo_note: Account<'info, TodoNote>,
    #[account(mut)]
    pub note_writer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TodoInstruction<'info> {
    pub writer: Signer<'info>,
    #[account(mut)]
    pub todo_note: Account<'info, TodoNote>,
}

#[account]
pub struct TodoNote {
    todos: Vec<Todo>,
    writer: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct Todo {
    is_finished: bool,
    task: String,
}

impl TodoNote {
    pub const MAX_SIZE: usize = 20/*: Pubkey*/ + 4/*: vec*/ + (1/*: is_finished*/ + (4 + 50)/*: task*/) * 20;
    
    pub fn add_todo(&mut self, task: String) -> Result<()> {
        if task.as_bytes().len() > 50 {
            Err(TodoError::TaskSizeTooBig.into())
        } else {
            self.todos.push(Todo { task, is_finished: false });
            Ok(())
        }
    }

    pub fn remove_todo(&mut self, task: String) -> Result<()> {
        match self.todos.iter().position(|x| x.task == task) {
            Some(i) => { self.todos.remove(i); }
            None => { return Err(TodoError::TaskNotFound.into()); }
        }

        Ok(())
    }

    pub fn update_todo(&mut self, from: Todo, to: Todo) -> Result<()> {
        match self.todos.iter().position(|x| { x.task == from.task && x.is_finished == from.is_finished }) {
            Some(i) => { 
                let target = self.todos.get_mut(i).unwrap();
                target.task = to.task;
                target.is_finished = to.is_finished;
            }
            None => { return Err(TodoError::TaskNotFound.into()); }
        }

        Ok(())
    }
}

#[error_code]
pub enum TodoError {
    TaskSizeTooBig,
    TaskNotFound,
}

