# Domino Game Implementation Details

## Enums
### `Color`
- **Description**: Represents the color of a domino. It includes five possible values: White, Blue, Green, Red, and Yellow.

## Variants
- White
- Blue
- Green
- Red
- Yellow

## Structures
### `Domino`
- **Description**: Represents a domino tile with two colors.

### Fields

- **0**: The color on the left side of the domino.
- **1**: The color on the right side of the domino.

### Methods

- **`new(left: Color, right: Color) -> Self`**: Creates a new Domino with the specified colors for the left and right sides.
- **`generate_color_code(number: i32) -> Color`**: Generates a Color based on an integer input. The integer maps to a specific color.
- **`split_hand(shuffled_dominos: &[Domino], mid_point: usize) -> (Vec<Domino>, Vec<Domino>)`**: Splits the shuffled dominoes into two hands at the given midpoint.
- **`count(dominos_set: &Vec<Domino>) -> i32`**: Returns the count of dominoes in the given set.
- **`display_domino(dominos: &[Domino])`**: Prints each domino in the list with its colors.

### `Player`
- **Description**: Represents a player in the game with a set of dominoes.

### Fields

- **dominos**: A vector containing the player's dominoes.

### Methods

- **`new(dominos: Vec<Domino>) -> Self`**: Creates a new Player with the specified set of dominoes.

### `Computer` 
- **Description**: Represents the computer-controlled player in the game with a set of dominoes.

### Fields

- **dominos**: A vector containing the computer's dominoes.
  
### Methods

- **`new(dominos: Vec<Domino>) -> Self`**: Creates a new Computer with the specified set of dominoes.
  
### `Game`
- **Description**: Manages the state of the game, including player and computer actions, and game status.

### Fields

- **player**: An instance of Player.
- **computer**: An instance of Computer.
- **state**: The current state of the game, represented by GameState.
- **starting_domino**: The current domino used to start the game or round.

### Methods

- **`new(player: Player, computer: Computer, starting_domino: Domino) -> Self`**: Creates a new Game with the specified player, computer, and starting domino.
- **`start_game() -> Game`**: Initializes and starts a new game, shuffling and distributing dominoes, and setting up the starting domino.
- **`is_valid_move(&self, domino: &Domino) -> bool`**: Checks if the provided domino can be played based on its color matching the starting domino.
- **`play_turn(&mut self)`**: Manages the turns of the player and computer, checking for valid moves and updating game state.
- **`choose_domino(&mut self)`**: Handles the player's turn, including input and validation of the chosen domino.
- **`computer_choose_domino(&mut self)`**: Handles the computer's turn, randomly selecting a valid domino.
- **`check_victory_conditions(&self) -> GameState`**: Checks if the game is won by either the player or computer.
- **`end_game(&self)`**: Displays the game result based on the final state.

### Functions

### `print_status`
- **Description**: Displays the current status of the game (e.g., player and computer's remaining dominoes).

### `win`
- **Description**: Displays a "You Win!" message when the player wins (handled by the end_game method).

### `print_game_over_screen`
- **Description**: Displays the "GAME OVER" screen and options to restart or quit (handled by the end_game method).

