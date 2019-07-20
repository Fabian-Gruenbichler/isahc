use mockito::{Matcher, mock, server_url};

speculate::speculate! {
    before {
        env_logger::try_init().ok();
    }

    test "accept headers populated by default" {
        let m = mock("GET", "/")
            .match_header("accept", "*/*")
            .match_header("accept-encoding", "deflate, gzip")
            .create();

        chttp::get(server_url()).unwrap();

        m.assert();
    }

    test "user agent contains expected format" {
        let m = mock("GET", "/")
            .match_header("user-agent", Matcher::Regex(r"^curl/\S+ chttp/\S+$".into()))
            .create();

        chttp::get(server_url()).unwrap();

        m.assert();
    }
}
