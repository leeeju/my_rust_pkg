use rclrs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ROS 2 초기화
    let context = Context::new([])?;
    // 노드 생성
    let node = Node::new(&context, "my_rust_node")?;

    // 타이머를 설정하여 주기적으로 메시지를 출력
    let timer = node.create_wall_timer(Duration::from_secs(1), move || {
        println!("Hello, ROS 2 from Rust!");
    })?;

    // 노드 실행
    rclrs::spin(&node)?;

    Ok(())
}
