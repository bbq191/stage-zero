package ikate.me.controller;

import cn.hutool.core.date.DateUtil;
import ikate.me.api.PayFeignApi;
import ikate.me.entities.PayDTO;
import ikate.me.enums.ReturnCodeEnum;
import ikate.me.resp.ResultData;
import jakarta.annotation.Resource;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class OrderController {
  @Resource private PayFeignApi payFeignApi;

  @GetMapping("/feign/pay/add")
  public ResultData addOrder(PayDTO payDTO) {
    System.out.println("假设下单成功，下面处理支付");
    return payFeignApi.addPay(payDTO);
  }

  @GetMapping("/feign/pay/del/{id}")
  public ResultData deleteOrder(@PathVariable("id") Integer id) {
    return payFeignApi.deletePay(id);
  }

  @GetMapping("/feign/pay/update")
  public ResultData updateOrder(PayDTO payDTO) {
    payFeignApi.updatePay(payDTO);
    return ResultData.success("成功返回记录，返回值" + payDTO);
  }

  @GetMapping("/feign/pay/get/{id}")
  public ResultData getPayInfo(@PathVariable("id") Integer id) {
    System.out.println("-------支付微服务远程调用，按照id查询订单支付流水信息");
    ResultData resultData = null;
    try {
      System.out.println("调用开始-----:" + DateUtil.now());
      resultData = payFeignApi.getPay(id);
    } catch (Exception e) {
      e.printStackTrace();
      System.out.println("调用结束-----:" + DateUtil.now());
      ResultData.fail(ReturnCodeEnum.RC500.getCode(), e.getMessage());
    }
    return resultData;
  }

  @GetMapping("/feign/mylb")
  public String mylb() {
    return payFeignApi.mylb();
  }
}
