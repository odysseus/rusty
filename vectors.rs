fn main() {
  let your_favorite_numbers = &[1,2,3];
  let my_favorite_numbers = &[4,5,6];

  let mut our_favorite_numbers = &[];
  our_favorite_numbers.push_all(&your_favorite_numbers);
  our_favorite_numbers.push_all(&my_favorite_numbers);
}
