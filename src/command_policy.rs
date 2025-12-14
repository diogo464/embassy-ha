/// Determines how an entity handles commands received from Home Assistant.
///
/// This policy controls whether an entity automatically publishes its state when it receives
/// a command from Home Assistant, or if the application should handle state updates manually.
///
/// # Variants
///
/// ## `PublishState` (Default)
///
/// When a command is received from Home Assistant, the entity automatically:
/// 1. Updates its internal state to match the command value
/// 2. Publishes the new state back to Home Assistant
///
/// This is useful for simple entities where the command should immediately be reflected as the
/// current state, such as:
/// - A switch that turns on/off immediately when commanded
/// - A number input that updates its value when changed in the UI
///
/// ## `Manual`
///
/// When a command is received from Home Assistant, the entity:
/// 1. Stores the command for the application to read via `wait()` or `command()`
/// 2. Does NOT automatically update or publish the state
///
/// The application must manually update the entity's state after processing the command.
/// This is useful when:
/// - The command triggers an action that may fail (e.g., turning on a motor)
/// - The actual state may differ from the commanded state
/// - You need to validate or transform the command before applying it
///
/// # Examples
///
/// ## Auto-publish (default)
///
/// ```no_run
/// # use embassy_ha::{CommandPolicy, SwitchConfig};
/// let config = SwitchConfig {
///     command_policy: CommandPolicy::PublishState, // or just use default
///     ..Default::default()
/// };
/// // When Home Assistant sends "ON", the switch state automatically becomes "ON"
/// ```
///
/// ## Manual control
///
/// ```no_run
/// # use embassy_ha::{CommandPolicy, SwitchConfig, BinaryState, Switch};
/// # async fn example(mut switch: Switch<'_>) {
/// let config = SwitchConfig {
///     command_policy: CommandPolicy::Manual,
///     ..Default::default()
/// };
///
/// loop {
///     let command = switch.wait().await;
///
///     // Try to perform the action
///     if turn_on_motor().await.is_ok() {
///         // Only update state if the action succeeded
///         switch.set(command);
///     }
/// }
/// # }
/// # async fn turn_on_motor() -> Result<(), ()> { Ok(()) }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandPolicy {
    /// Automatically publish the entity's state when a command is received.
    PublishState,

    /// Do not automatically publish state. The application must manually update the state.
    Manual,
}

impl Default for CommandPolicy {
    fn default() -> Self {
        Self::PublishState
    }
}
