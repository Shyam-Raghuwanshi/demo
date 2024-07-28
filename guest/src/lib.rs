use wit::demo::utils::complex::Num;

mod wit {
    use wit_bindgen::generate;
    generate!({
        path:"../wit",
        world:"reactor",
    });

    use super::Guest;
    export!(Guest);
    pub use self::exports::demo::utils::inbound_http::*;
}
struct Guest;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

impl wit::exports::demo::utils::inbound_http::Guest for Guest {
    fn handle_request(req: wit::Request) -> wit::Response {
        println!("Request {:?}", req);
        let sum =
            wit::demo::utils::operation::add(Num { real: 10, imag: 55 }, Num { real: 20, imag: 10 });
        // let sum = wit::demo::utils::operation::add(
        //     wit::demo::utils::complex::Num { real: 1, imag: 1 },
        //     wit::demo::utils::complex::Num { real: 2, imag: 2 },
        // );
        println!("Sum is = {:?}", sum);
        wit::Response {
            status: 200,
            headers: None,
            body: None,
        }
    }
}
