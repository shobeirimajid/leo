// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

// local program commands
pub mod build;
pub use build::Build;

pub mod clean;
pub use clean::Clean;

pub mod deploy;
pub use deploy::Deploy;

pub mod new;
pub use new::New;

// pub mod node;
// pub use node::Node;

pub mod run;
pub use run::Run;

use crate::context::*;
use leo_errors::Result;

use tracing::span::Span;

pub(crate) type Network = snarkvm::prelude::Testnet3;
pub(crate) const ALEO_CLI_COMMAND: &str = "aleo";

/// Base trait for the Leo CLI, see methods and their documentation for details.
pub trait Command {
    /// If the current command requires running another command beforehand
    /// and needs its output result, this is where the result type is defined.
    /// Example: type Input: <CommandA as Command>::Out
    type Input;

    /// Defines the output of this command, which may be used as `Input` for another
    /// command. If this command is not used as a prelude for another command,
    /// this field may be left empty.
    type Output;

    /// Adds a span to the logger via `tracing::span`.
    /// Because of the specifics of the macro implementation, it is not possible
    /// to set the span name with a non-literal i.e. a dynamic variable even if this
    /// variable is a &'static str.
    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    /// Runs the prelude and returns the Input of the current command.
    fn prelude(&self, context: Context) -> Result<Self::Input>
    where
        Self: std::marker::Sized;

    /// Runs the main operation of this command. This function is run within
    /// context of 'execute' function, which sets logging and timers.
    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output>
    where
        Self: std::marker::Sized;

    /// A wrapper around the `apply` method.
    /// This function sets up tracing, timing, and the context.
    fn execute(self, context: Context) -> Result<Self::Output>
    where
        Self: std::marker::Sized,
    {
        let input = self.prelude(context.clone())?;

        // Create the span for this command.
        let span = self.log_span();
        let span = span.enter();

        // Calculate the execution time for this command.
        let out = self.apply(context, input);

        drop(span);

        out
    }

    /// Executes command but empty the result. Comes in handy where there's a
    /// need to make match arms compatible while keeping implementation-specific
    /// output possible. Errors however are all of the type Error
    fn try_execute(self, context: Context) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        self.execute(context).map(|_| Ok(()))?
    }
}
