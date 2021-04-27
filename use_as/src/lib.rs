mod front_of_house;

use crate::front_of_house::hosting as host;

pub fn eat_at_restaurant() {
    host::add_wait_list();
    host::add_wait_list();
    host::add_wait_list();
}