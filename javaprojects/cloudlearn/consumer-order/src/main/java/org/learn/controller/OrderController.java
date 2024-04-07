package org.learn.controller;

import org.learn.entities.PayDTO;
import org.learn.resp.ResultData;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.RestTemplate;

import java.util.ArrayList;
import java.util.List;

@RestController
public class OrderController {
  //  public static final String PaymentSrv_URL = "http://localhost:8001"; // 先写死，硬编码

  public static final String PaymentSrv_URL = "http://payment-service"; // 服务注册中心上的微服务名称
  @Autowired private RestTemplate restTemplate;

  /**
   * 一般情况下，通过浏览器的地址栏输入url，发送的只能是get请求 我们底层调用的是post方法，模拟消费者发送get请求，客户端消费者 参数可以不添加@RequestBody
   *
   * @param payDTO
   * @return
   */
  @GetMapping("/consumer/pay/add")
  public ResultData addOrder(PayDTO payDTO) {
    return restTemplate.postForObject(PaymentSrv_URL + "/pay/add", payDTO, ResultData.class);
  }

  @GetMapping("/consumer/pay/del/{id}")
  public ResultData<String> deleteOrder(@PathVariable("id") Integer id) {
    restTemplate.delete(PaymentSrv_URL + "/pay/del/" + id);
    return ResultData.success("成功返回记录，返回值" + id);
  }

  @GetMapping("/consumer/pay/update")
  public ResultData<String> updateOrder(PayDTO payDTO) {
    restTemplate.put(PaymentSrv_URL + "/pay/update", payDTO);
    return ResultData.success("成功返回记录，返回值" + payDTO);
  }

  @GetMapping("/consumer/pay/get/{id}")
  public ResultData getPayInfo(@PathVariable("id") Integer id) {
    return restTemplate.getForObject(PaymentSrv_URL + "/pay/get/" + id, ResultData.class, id);
  }

  @GetMapping("/consumer/pay/getall")
  public ResultData getAll() {
    List<PayDTO> all = new ArrayList<>();
    return restTemplate.getForObject(PaymentSrv_URL + "/pay/getall", ResultData.class, all);
  }
}
