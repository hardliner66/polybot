
fn main() {
    let conn = data_server::establish_connection();

    data_server::create_user(&conn, "iamhardliner");
}
