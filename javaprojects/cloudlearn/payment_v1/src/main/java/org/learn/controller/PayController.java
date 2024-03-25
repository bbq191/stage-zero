package org.learn.controller;

import cn.hutool.core.bean.BeanUtil;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.learn.entities.Pay;
import org.learn.entities.PayDTO;
import org.learn.service.PayService;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@Slf4j
public class PayController {
  @Resource private PayService payService;

  @PostMapping(value = "/pay/add")
  public String addPay(@RequestBody Pay pay) {
    System.out.println(pay.toString());
    int i = payService.add(pay);
    return "成功返回记录，返回值" + i;
  }

  @DeleteMapping(value = "/pau/del/{id}")
  public Integer deletePay(@PathVariable("id") Integer id) {
    return payService.delete(id);
  }

  @PutMapping(value = "/pay/update")
  public String updatePay(@RequestBody PayDTO payDTO) {
    Pay pay = new Pay();
    BeanUtil.copyProperties(payDTO, pay);
    int i = payService.update(pay);
    return "成功返回记录，返回值" + i;
  }

  @GetMapping(value = "/pay/get/{id}")
  public Pay getById(@PathVariable("id") Integer id) {
    return payService.getById(id);
  }

  @GetMapping(value = "/pay/getall")
  public List<Pay> getAll() {
    return payService.getAll();
  }
}
