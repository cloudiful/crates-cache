use crate::{Cache, StoringMethod};

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Apple {
        pub(crate) name: String,
        price: i32,
    }

    impl Apple {
        fn new(price: i32) -> Apple {
            Apple {
                name: String::from("water"),
                price,
            }
        }
    }

    #[test]
    fn json() {
        save_to_file();
        read_from_file();
        remove();
    }
    fn save_to_file() {
        let apple = Apple::new(4);

        let mut apple_cache = Cache::new("apple2");
        apple_cache.set_storing_method(StoringMethod::JSON);

        apple_cache.save(&apple);
    }
    fn read_from_file() {
        let mut apple_cache = Cache::new("apple2");
        apple_cache.set_storing_method(StoringMethod::JSON);

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(Some(Apple::new(4)), cached_apple);
    }
    fn remove() {
        let apple = Apple::new(3);

        let mut apple_cache = Cache::new("apple1");
        apple_cache.set_storing_method(StoringMethod::JSON);

        apple_cache.save(&apple);

        apple_cache.remove();

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(None, cached_apple);
    }

    #[test]
    fn sqlite() {
        save_to_sqlite();
        read_from_sqlite();
        clear_sqlite();
        change_table();
    }

    fn save_to_sqlite() {
        let mut apple = Apple::new(5);

        apple.price = 7;

        let apple_cache = Cache::new("apple3");

        apple_cache.save(&apple);

        apple.price = 9;
    }
    fn read_from_sqlite() {
        let apple_cache = Cache::new("apple3");

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(Some(Apple::new(7)), cached_apple);
    }
    fn clear_sqlite() {
        let apple = Apple::new(3);

        let apple_cache = Cache::new("apple1");

        apple_cache.save(&apple);

        apple_cache.clear();

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(None, cached_apple);
    }
    fn change_table() {
        let mut apple_cache = Cache::new("apple3");
        apple_cache.set_table("data");
        let apple_none: Option<Apple> = apple_cache.read();
        assert_eq!(None, apple_none);

        apple_cache.save(&Apple::new(12312));
        let apple_some: Option<Apple> = apple_cache.read();
        assert_eq!(Some(Apple::new(12312)), apple_some);

        apple_cache.clear();
    }
}