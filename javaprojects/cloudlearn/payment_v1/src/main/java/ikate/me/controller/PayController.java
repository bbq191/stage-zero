package ikate.me.controller;

import cn.hutool.core.bean.BeanUtil;
import ikate.me.entities.Pay;
import io.swagger.v3.oas.annotations.tags.Tag;
import io.swagger.v3.oas.annotations.Operation;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import ikate.me.entities.PayDTO;
import ikate.me.resp.ResultData;
import ikate.me.service.PayService;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.concurrent.TimeUnit;

@RestController
@Slf4j
@Tag(name = "支付微服务模块", description = "支付CRUD")
public class PayController {
  @Resource private PayService payService;

  @PostMapping(value = "/pay/add")
  @Operation(summary = "新增", description = "新增支付流水方法,json串做参数")
  public ResultData<String> addPay(@RequestBody Pay pay) {
    System.out.println(pay.toString());
    int i = payService.add(pay);
    return ResultData.success("成功返回记录，返回值" + i);
  }

  @DeleteMapping(value = "/pay/del/{id}")
  @Operation(summary = "删除", description = "删除支付流水方法")
  public ResultData<Integer> deletePay(@PathVariable("id") Integer id) {
    int i = payService.delete(id);
    return ResultData.success(i);
  }

  @PutMapping(value = "/pay/update")
  @Operation(summary = "修改", description = "修改支付流水方法")
  public ResultData<String> updatePay(@RequestBody PayDTO payDTO) {
    Pay pay = new Pay();
    BeanUtil.copyProperties(payDTO, pay);
    int i = payService.update(pay);
    return ResultData.success("成功返回记录，返回值" + i);
  }

  @GetMapping(value = "/pay/get/{id}")
  @Operation(summary = "按照ID查流水", description = "查询支付流水方法")
  public ResultData<Pay> getById(@PathVariable("id") Integer id) {
    if (id < 0) throw new RuntimeException("id不能为负数");
    try {
      TimeUnit.SECONDS.sleep(62);
    } catch (InterruptedException e) {
      e.printStackTrace();
    }
    return ResultData.success(payService.getById(id));
  }

  @GetMapping(value = "/pay/getall")
  @Operation(summary = "查所有流水", description = "查询所有支付流水方法")
  public ResultData<List<Pay>> getAll() {
    return ResultData.success(payService.getAll());
  }

  @Value("${server.port}")
  private String port;

  @GetMapping(value = "/pay/get/info")
  private String getInfoByConsul(@Value("${payment.info}") String paymentInfo) {
    return "paymentInfo: " + paymentInfo + "\t" + "port: " + port;
  }
}
