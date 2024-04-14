package ikate.me;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.client.discovery.EnableDiscoveryClient;
import org.springframework.cloud.context.config.annotation.RefreshScope;
import tk.mybatis.spring.annotation.MapperScan;

@SpringBootApplication
@EnableDiscoveryClient // 服务注册和发现
@RefreshScope // 动态刷新
@MapperScan("ikate.me.mapper")
public class Payment_v1 {
  public static void main(String[] args) {
    SpringApplication.run(Payment_v1.class, args);
  }
}
