package ikate.me.controller;

import jakarta.annotation.Resource;
import ikate.me.api.PayFeignApi;
import ikate.me.entities.PayDTO;
import ikate.me.resp.ResultData;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class OrderController {
  @Resource private PayFeignApi payFeignApi;

  @GetMapping("/feign/pay/add")
  public ResultData addOrder(PayDTO payDTO) {
    return payFeignApi.addPay(payDTO);
  }

  @GetMapping("/feign/pay/del/{id}")
  public ResultData deleteOrder(@PathVariable("id") Integer id) {
    return payFeignApi.deletePay(id);
    //    return ResultData.success("成功返回记录，返回值" + id);
  }

  @GetMapping("/feign/pay/update")
  public ResultData updateOrder(PayDTO payDTO) {
    payFeignApi.updatePay(payDTO);
    return ResultData.success("成功返回记录，返回值" + payDTO);
  }

  @GetMapping("/feign/pay/get/{id}")
  public ResultData getPayInfo(@PathVariable("id") Integer id) {
    return payFeignApi.getPay(id);
  }

  @GetMapping("/feign/mylb")
  public String mylb() {
    return payFeignApi.mylb();
  }
}
