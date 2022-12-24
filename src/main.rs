use std::net::TcpListener;
fn main() {
    // 监听 TCP 连接
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // incoming 方法会返回一个产生流序列的迭代器
    for stream in listener.incoming() {
        // 单个流 stream 代表了一个在客户端和服务器之间打开的连接
        // connection 则代表了客户端连接服务器、服务器生成响应，以及服务器关闭连接的全部请求与响应过程
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
