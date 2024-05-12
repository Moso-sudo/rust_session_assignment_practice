fn main() {
    const AGE : i32 =13;
    #[derive(Debug)]

    enum STATUS {
        ADULT,
        CHILD,
    }
    let user_status: STATUS = if AGE>18{
        STATUS::ADULT
    }else{
        STATUS::CHILD
    };
    println!("the status is {:?}", user_status)

}
