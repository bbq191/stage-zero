package org.learn;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.client.discovery.EnableDiscoveryClient;
import tk.mybatis.spring.annotation.MapperScan;

@SpringBootApplication
@EnableDiscoveryClient
@MapperScan("org.learn.mapper")
public class Payment_v1 {
  public static void main(String[] args) {
    SpringApplication.run(Payment_v1.class, args);
  }
}
