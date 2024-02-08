// trait SimpleFuture {
//     type Output;
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
// }

// enum Poll<T> {
//     Ready(T),
//     Pending,
// }

// //Future 能通过调用 poll 的方式推进，这会尽可能地推进 future 到完成状态。如果 future 完成了， 那就会返回 poll::Ready(result)。如果 future 尚未完成，则返回 poll::Pending，并且安排 wake() 函数在 Future 准备好进一步执行时调用（译者注：注册回调函数）。当 wake() 调用 时，驱动 Future 的执行器会再次 poll 使得 Future 有所进展。

// pub struct SocketRead<'a> {
//     socket: &'a Socket,
// }

// impl SimpleFuture for SocketRead<'_> {
//     type Output = Vec<u8>;
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         if self.socket.has_data_to_read() {
//             // socket有数据，写入buffer中并返回
//             Poll::Ready(self.socket.read_buf())
//         } else {
//             // socket中还没数据
//             //
//             // 注册一个`wake`函数，当数据可用时，该函数会被调用，
//             // 然后当前Future的执行器会再次调用`poll`方法，此时就可以读取到数据
//             self.socket.set_readable_callback(wake);
//             Poll::Pending
//         }
//     }
// }
