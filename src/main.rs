use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{collections::HashMap, fmt};

use console::style;
use dialoguer::{Confirm, Select};

struct PlayRecord {
  open_card: Card,
  closed_card: Card,
  bought: bool,
}

struct WinRecords {
  win_records: Vec<PlayRecord>,
  lose_records: Vec<PlayRecord>,
}
fn percentage_bar(percentage: f64, max_percentage: f64, scale: u32) -> (f64, String) {
  let mut bar = String::new();
  let normal_percentage = percentage / max_percentage;
  let bar_length = (normal_percentage * scale as f64) as u32;
  for _ in 0..bar_length {
    bar.push('█');
  }
  // for _ in bar_length..scale {
  //   bar.push(' ');
  // }
  (percentage, bar)
}
impl WinRecords {
  fn new() -> WinRecords {
    WinRecords {
      win_records: Vec::new(),
      lose_records: Vec::new(),
    }
  }
  fn record_win(&mut self, hand: &Hand) {
    let open_card = hand.cards[0].clone();
    let closed_card = hand.cards[1].clone();
    let bought = hand.cards.len() > 2;
    let record = PlayRecord {
      open_card,
      closed_card,
      bought,
    };
    self.win_records.push(record);
  }
  fn record_loss(&mut self, hand: &Hand) {
    let open_card = hand.cards[0].clone();
    let closed_card = hand.cards[1].clone();
    let bought = hand.cards.len() > 2;
    let record = PlayRecord {
      open_card,
      closed_card,
      bought,
    };
    self.lose_records.push(record);
  }
  fn display_stats(&self) {
    // Wins per open cards
    let mut wins_per_open_card = HashMap::<Value, u32>::new();
    let mut losses_per_open_card = HashMap::<Value, u32>::new();

    for record in &self.win_records {
      let value = record.open_card.value;
      let count = wins_per_open_card.entry(value).or_insert(0);
      *count += 1;
    }

    for record in &self.lose_records {
      let value = record.open_card.value;
      let count = losses_per_open_card.entry(value).or_insert(0);
      *count += 1;
    }
    // Sort the hasmaps by value
    let mut wins_per_open_card: Vec<_> = wins_per_open_card.into_iter().collect();
    wins_per_open_card.sort_by(|a, b| b.1.cmp(&a.1));
    let mut losses_per_open_card: Vec<_> = losses_per_open_card.into_iter().collect();
    losses_per_open_card.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Wins per open card:");
    let total_wins = self.win_records.len() as f64;
    for (value, count) in &wins_per_open_card {
      let (percentage, bar) = percentage_bar((*count as f64 / total_wins) * 100.0, 100.0, 200);
      println!(" {:>2}: {:>9} ({:>5.2}%) {}", value, count, percentage, bar);
    }

    println!("Losses per open card:");
    let total_losses = self.lose_records.len() as f64;
    for (value, count) in &losses_per_open_card {
      let (percentage, bar) = percentage_bar((*count as f64 / total_losses) * 100.0, 100.0, 200);
      println!(
        "  {:>2}: {:>9} ({:>5.2}%) {}",
        value, count, percentage, bar
      );
    }

    // Best open closed combination with bought
    let wins_with_bought = self
      .win_records
      .iter()
      .filter(|record| record.bought)
      .collect::<Vec<_>>();
    // Sort so biggest value card is open and smallest is closed
    let mut wins_with_bought: Vec<_> = wins_with_bought
      .iter()
      .map(|record| {
        let mut cards = vec![record.open_card.clone(), record.closed_card.clone()];
        cards.sort_by(|a, b| b.value.to_u32().cmp(&a.value.to_u32()));
        (cards[0].value, cards[1].value)
      })
      .collect();
    wins_with_bought.sort_by(|a, b| a.0.to_u32().cmp(&b.0.to_u32()));
    let mut wins_with_bought =
      wins_with_bought
        .into_iter()
        .fold(HashMap::<(Value, Value), u32>::new(), |mut acc, value| {
          let count = acc.entry(value).or_insert(0);
          *count += 1;
          acc
        });
    let mut wins_with_bought: Vec<_> = wins_with_bought.into_iter().collect();
    wins_with_bought.sort_by(|a, b| b.1.cmp(&a.1));
    wins_with_bought.truncate(40);
    println!("Best open closed combination with bought:");
    for ((open, closed), count) in &wins_with_bought {
      let mut hand = Hand::new();
      let mut open_card = Card::new(Suit::Hearts, *open);
      open_card.open();
      let mut closed_card = Card::new(Suit::Hearts, *closed);
      closed_card.open();
      hand.add_card(open_card);
      hand.add_card(closed_card);
      let (percentage, bar) = percentage_bar((*count as f64 / total_wins) * 100.0, 100.0, 1000);

      println!(
        " {:<20} {:<6} ({:>5.4}%) {}",
        format!("{}", hand),
        count,
        percentage,
        bar
      );
    }

    // Worst open closed combination with bought
    let losses_with_bought = self
      .lose_records
      .iter()
      .filter(|record| record.bought)
      .collect::<Vec<_>>();
    // Sort so biggest value card is open and smallest is closed
    let mut losses_with_bought: Vec<_> = losses_with_bought
      .iter()
      .map(|record| {
        let mut cards = vec![record.open_card.clone(), record.closed_card.clone()];
        cards.sort_by(|a, b| b.value.to_u32().cmp(&a.value.to_u32()));
        (cards[0].value, cards[1].value)
      })
      .collect();
    losses_with_bought.sort_by(|a, b| a.0.to_u32().cmp(&b.0.to_u32()));
    let mut losses_with_bought = losses_with_bought.into_iter().fold(
      HashMap::<(Value, Value), u32>::new(),
      |mut acc, value| {
        let count = acc.entry(value).or_insert(0);
        *count += 1;
        acc
      },
    );
    let mut losses_with_bought: Vec<_> = losses_with_bought.into_iter().collect();
    losses_with_bought.sort_by(|a, b| b.1.cmp(&a.1));
    losses_with_bought.truncate(40);
    println!("Worst open closed combination with bought:");
    for ((open, closed), count) in &losses_with_bought {
      let mut hand = Hand::new();
      let mut open_card = Card::new(Suit::Hearts, *open);
      open_card.open();
      let mut closed_card = Card::new(Suit::Hearts, *closed);
      closed_card.open();
      hand.add_card(open_card);
      hand.add_card(closed_card);
      let (percentage, bar) = percentage_bar((*count as f64 / total_losses) * 100.0, 100.0, 1000);

      println!(
        " {:<20} {:<6} ({:>5.4}%) {}",
        format!("{}", hand),
        count,
        percentage,
        bar
      );
    }
  }
}

fn main() {
  println!("Hello, world!");
  let bank = Player::new(0);
  println!("Bank: {bank}");
  let player1 = Player::new(1);
  println!("{player1}");
  let player2 = Player::new(2);
  println!("{player2}");
  let player3 = Player::new(3);
  let player4 = Player::new(4);

  let mut game = Game::new(bank);
  game.add_player(player1);
  game.add_player(player2);
  game.add_player(player3);
  game.add_player(player4);
  println!("");
  println!("GAME STATE: \n{game}");
  let mut games_won_1 = 0;
  let mut games_won_2 = 0;
  let mut games_won_bank = 0;
  let rounds = 10_000_000;
  let bar = indicatif::ProgressBar::new(rounds);
  for round in 0..rounds {
    bar.inc(1);
    // let should_continue = Confirm::new()
    //   .with_prompt("New round?")
    //   .default(true)
    //   .interact()
    //   .unwrap();
    // if !should_continue {
    //   break;
    // }
    game.start_round();
    // println!("");
    // println!("GAME STATE: \n{game}");
    let players = game.players.clone();
    for player in players {
      if player.bid == 0 {
        continue;
      }
      game.play_player(&player);
      // println!("\n");
    }
    game.play_bank();
    // println!("GAME STATE: \n{game}");

    if game.players.iter().find(|p| p.id == 1).unwrap().money > 100 {
      games_won_1 += 1;
    } else {
      games_won_bank += 1;
    }
    if game.players.iter().find(|p| p.id == 2).unwrap().money > 100 {
      games_won_2 += 1;
    } else {
      games_won_bank += 1;
    }
    game.clean_up();
  }
  bar.finish();
  println!("\n\n");
  println!(
    "Games won by player 1: {games_won_1:>6} ({percentage:.2}%)",
    games_won_1 = games_won_1,
    percentage = (games_won_1 as f64 / rounds as f64) * 100.0
  );
  println!(
    "Games won by player 2: {games_won_2:>6} ({percentage:.2}%)",
    games_won_2 = games_won_2,
    percentage = (games_won_2 as f64 / rounds as f64) * 100.0
  );
  println!(
    "Games won by bank:     {games_won_bank:>6} ({percentage:.2}%)",
    games_won_bank = games_won_bank,
    percentage = (games_won_bank as f64 / (rounds as f64 * 2 as f64)) * 100.0
  );

  game.records.display_stats();
}

struct Game {
  deck: Deck,
  bank: Player,
  players: Vec<Player>,
  bank_rounds_played: u32,
  records: WinRecords,
}

struct Deck {
  cards: Vec<Card>,
}

impl Deck {
  fn new() -> Deck {
    let mut deck = Vec::new();
    for suit in vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
      for value in vec![
        Value::Ace,
        Value::Two,
        Value::Three,
        Value::Four,
        Value::Five,
        Value::Six,
        Value::Seven,
        Value::Eight,
        Value::Nine,
        Value::Ten,
        Value::Jack,
        Value::Queen,
        Value::King,
      ] {
        deck.push(Card::new(suit, value));
      }
    }
    Deck { cards: deck }
  }

  fn shuffle(&mut self) {
    let mut rng = thread_rng();
    self.cards.shuffle(&mut rng);
  }

  fn draw(&mut self) -> Card {
    self.cards.pop().expect("No more cards in deck")
  }

  fn expected_value(&self) -> f64 {
    self
      .cards
      .iter()
      .fold(0.0, |acc, card| acc + card.worth() as f64)
      / self.cards.len() as f64
  }
}

impl Game {
  fn new(bank: Player) -> Game {
    let mut deck = Deck::new();
    deck.shuffle();
    Game {
      deck: deck,
      bank: bank,
      players: Vec::new(),
      bank_rounds_played: 0,
      records: WinRecords::new(),
    }
  }
  fn add_player(&mut self, player: Player) {
    self.players.push(player);
  }
  fn start_round(&mut self) {
    // Shuffle players
    let mut rng = thread_rng();
    self.players.shuffle(&mut rng);

    // Deal open card
    for player in &mut self.players {
      let mut card = self.deck.draw();
      card.open();
      player.hand.add_card(card);
    }
    let mut card = self.deck.draw();
    card.open();
    self.bank.hand.add_card(card);

    // Let players place their bid
    for player in &mut self.players {
      let placed_bid = generate_bid(player);
      player.bid = placed_bid;
      player.money -= placed_bid;
      self.bank.money -= placed_bid;
    }

    // Deal closed cards
    for player in &mut self.players {
      let mut card = self.deck.draw();
      card.open_for_owner();
      player.hand.add_card(card);

      loop {
        if player.hand.cards.iter().any(|c| c.value == Value::Seven)
          && player.hand.cards.iter().any(|c| c.value == Value::Eight)
        {
          player.hand.cards.clear();
          let mut card1 = self.deck.draw();
          card1.open();
          player.hand.add_card(card1);

          let mut card2 = self.deck.draw();
          card2.open_for_owner();
          player.hand.add_card(card2);
        } else {
          break;
        }
      }
    }

    let card = self.deck.draw();
    self.bank.hand.add_card(card);
  }
  fn play_player(&mut self, player: &Player) {
    let player = &mut self
      .players
      .iter_mut()
      .find(|p| p.id == player.id)
      .expect(format!("Player with id {} does not exist", player.id).as_str());

    if player.bid == 0 {
      panic!("Player {} has no bid, but is still being played", player.id);
    }

    // println!("Playing player {}", player.id);
    loop {
      let possible_values = player.hand.possible_values();
      let greatest_value = player.hand.greatest_value();
      let smallest_value = player.hand.smallest_value();

      // println!("  Possible values: {values:?}", values = possible_values);
      if smallest_value > 21 {
        // println!("  Bust!");
        self.bank.money += player.bid * 2;
        player.bid = 0;
        self.records.record_loss(&player.hand);
        break;
      } else if player.hand.possible_values().contains(&21) {
        // println!("  Blackjack!");
        player.money += player.bid * 2;
        player.bid = 0;
        self.records.record_win(&player.hand);
        break;
      }

      // println!("  Greatest value: {value}", value = greatest_value);
      match player.id {
        1 => {
          let deck = &self.deck;
          let expected_value = deck.expected_value();
          if greatest_value as f64 + expected_value > 21.0 {
            // println!("  Staying A");
            break;
          } else {
            let mut card = self.deck.draw();
            card.open();
            // println!("  Drawing a card: {card}", card = card);
            player.hand.add_card(card);
          }
        }
        _ => {
          let expected_value = 6.0;
          if smallest_value as f64 + expected_value > 21.0 {
            // println!("  Staying B");
            break;
          } else {
            let mut card = self.deck.draw();
            card.open();
            // println!("  Drawing a card: {card}", card = card);
            player.hand.add_card(card);
          }
        }
        // 3 => {
        //   if greatest_value >= 17 {
        //     // println!("  Staying");
        //     break;
        //   } else {
        //     let mut card = self.deck.draw();
        //     card.open();
        //     // println!("  Drawing a card: {card}", card = card);
        //     player.hand.add_card(card);
        //   }
        // }
        // _ => panic!("Player id {} does not exist", player.id),
      }
      if player.hand.cards.len() > 6 {
        // println!("  player won by cards");
        player.money += player.bid * 2;
        self.records.record_win(&player.hand);
        player.bid = 0;
        break;
      }
    }
  }
  fn play_bank(&mut self) {
    self.bank_rounds_played += 1;
    // println!("Playing bank");
    self.bank.hand.cards.iter_mut().for_each(|f| f.open());
    loop {
      let bank = &mut self.bank;

      // println!(
      //   "  Possible values: {values:?}",
      //   values = bank.hand.possible_values()
      // );
      if bank.hand.smallest_value() > 21 {
        // println!(
        //   "  Bank died after {rounds} rounds",
        //   rounds = self.bank_rounds_played
        // );
        self.bank_rounds_played = 0;
        for player in &mut self.players {
          if player.bid == 0 {
            continue;
          }
          player.money += player.bid * 2;
          self.records.record_win(&player.hand);
          player.bid = 0;
        }
        break;
      }

      let greatest_value = bank.hand.greatest_value();
      // println!("  Greatest value: {value}", value = greatest_value);
      if greatest_value >= 17 {
        // println!("  Staying");
        for player in &mut self.players {
          if player.bid == 0 {
            continue;
          }
          let player_value = player.hand.greatest_value();
          if player_value > greatest_value {
            // println!("  Player {} wins against the bank!", player.id);
            player.money += player.bid * 2;
            self.records.record_win(&player.hand);
            player.bid = 0;
          } else {
            // println!("  Player {} loses against the bank!", player.id);
            self.bank.money += player.bid * 2;
            self.records.record_loss(&player.hand);
            player.bid = 0;
          }
        }
        break;
      } else {
        // println!("  Drawing");
        let mut card = self.deck.draw();
        card.open();
        bank.hand.add_card(card);
      }
    }
  }
  fn clean_up(&mut self) {
    self.players.iter_mut().for_each(|p| p.hand.cards.clear());
    self.players.iter_mut().for_each(|p| p.money = 100);

    self.bank.hand.cards.clear();
    self.bank.money = 100;
    self.deck = Deck::new();
    self.deck.shuffle();
  }
}

fn generate_bid(player: &Player) -> u32 {
  let mut bid = 1;
  bid
}
#[derive(Clone)]
struct Player {
  id: u32,
  money: u32,
  bid: u32,
  hand: Hand,
}

impl Player {
  fn new(id: u32) -> Player {
    Player {
      id: id,
      money: 100,
      bid: 0,
      hand: Hand::new(),
    }
  }
}
#[derive(Clone)]
struct Hand {
  cards: Vec<Card>,
}

impl Hand {
  fn new() -> Hand {
    Hand { cards: Vec::new() }
  }

  fn add_card(&mut self, card: Card) {
    self.cards.push(card);
  }

  fn possible_values(&self) -> Vec<u32> {
    let mut values: Vec<u32> = Vec::new();
    let mut aces = 0;
    let mut total = 0;
    for card in &self
      .cards
      .iter()
      .filter(|c| c.visibility != Visibility::None)
      .collect::<Vec<_>>()
    {
      match card.value {
        Value::Ace => aces += 1,
        _ => total += card.worth(),
      }
    }
    values.push(total);
    for _ in 0..aces {
      for i in 0..values.len() {
        if values[i] + 11 <= 21 {
          values.push(values[i] + 1);
          values[i] += 11;
        } else {
          values[i] += 1;
        }
      }
    }
    values
    // .iter()
    // .filter(|v| v.clone() <= &21u32)
    // .map(|v| v.clone())
    // .collect()
  }
  fn greatest_value(&self) -> u32 {
    self.possible_values().iter().max().unwrap().clone()
  }
  fn smallest_value(&self) -> u32 {
    self.possible_values().iter().min().unwrap().clone()
  }
}

#[derive(PartialEq, Clone, Copy)]
enum Visibility {
  All,
  Players,
  Owner,
  Bank,
  None,
}

#[derive(PartialEq, Clone, Copy)]
enum Suit {
  Spades,
  Hearts,
  Diamonds,
  Clubs,
}
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Value {
  Ace,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
}
impl Value {
  fn to_u32(&self) -> u32 {
    match self {
      Value::Ace => 1,
      Value::Two => 2,
      Value::Three => 3,
      Value::Four => 4,
      Value::Five => 5,
      Value::Six => 6,
      Value::Seven => 7,
      Value::Eight => 8,
      Value::Nine => 9,
      Value::Ten => 10,
      Value::Jack => 11,
      Value::Queen => 12,
      Value::King => 13,
    }
  }
}

#[derive(Clone)]
struct Card {
  suit: Suit,
  value: Value,
  visibility: Visibility,
}

impl Card {
  fn new(suit: Suit, value: Value) -> Card {
    Card {
      suit: suit,
      value: value,
      visibility: Visibility::None,
    }
  }

  fn open(&mut self) {
    self.visibility = Visibility::All;
  }

  fn open_for_owner(&mut self) {
    self.visibility = Visibility::Owner;
  }

  fn worth(&self) -> u32 {
    match self.value {
      Value::Ace => 1,
      Value::Two => 2,
      Value::Three => 3,
      Value::Four => 4,
      Value::Five => 5,
      Value::Six => 6,
      Value::Seven => 7,
      Value::Eight => 8,
      Value::Nine => 9,
      Value::Ten => 10,
      Value::Jack => 10,
      Value::Queen => 10,
      Value::King => 10,
    }
  }
}

impl fmt::Display for Suit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let suit = match self {
      Suit::Spades => "♠",
      Suit::Hearts => "♥",
      Suit::Diamonds => "♦",
      Suit::Clubs => "♣",
    };
    write!(f, "{}", suit)
  }
}
impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let value = match self {
      Value::Ace => " A",
      Value::Two => " 2",
      Value::Three => " 3",
      Value::Four => " 4",
      Value::Five => " 5",
      Value::Six => " 6",
      Value::Seven => " 7",
      Value::Eight => " 8",
      Value::Nine => " 9",
      Value::Ten => "10",
      Value::Jack => " J",
      Value::Queen => " Q",
      Value::King => " K",
    };
    write!(f, "{}", value)
  }
}
impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let visibility = match self.visibility {
      Visibility::None => " XX".to_string(),
      _ => format!("{}{}", self.value, self.suit),
    };
    write!(f, "{}", visibility)
  }
}
impl fmt::Display for Hand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} ({})",
      &self
        .cards
        .iter()
        .map(|c| format!("{c}"))
        .collect::<Vec<_>>()
        .join(" "),
      self
        .possible_values()
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    )
  }
}
impl fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let hand = match self.hand.cards.len() {
      0 => "no cards".to_string(),
      _ => format!("{}", self.hand),
    };
    let bid = match self.bid {
      0 => "has not placed a bid".to_string(),
      _ => format!("bid {}$", self.bid),
    };
    write!(
      f,
      "Player {}: has {} and {} ({}$)",
      self.id, hand, bid, self.money
    )
  }
}
impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut players = String::new();
    for player in &self.players {
      players.push_str(&format!("  {}\n", player));
    }
    write!(f, "Bank:\n  {}\nPlayers:\n{}", self.bank, players)
  }
}
