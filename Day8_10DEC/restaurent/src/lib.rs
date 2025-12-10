mod front_of_house {
    mod hosting{
        fn add_to _waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order (){}
        fn serve_order (){
            crate :: front_of_house ::seat_at_table(); //absolute path
            super :: hosting :: seat_at_table() ; //relative path 
        }
        fn take_payment (){}
    }
}