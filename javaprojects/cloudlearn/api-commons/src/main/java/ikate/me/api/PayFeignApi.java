package ikate.me.api;

import ikate.me.entities.PayDTO;
import ikate.me.resp.ResultData;
import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.web.bind.annotation.*;

@FeignClient(value = "payment-service")
public interface PayFeignApi {
  @PostMapping(value = "/pay/add")
  public ResultData addPay(@RequestBody PayDTO payDTO);

  @PutMapping(value = "/pay/update")
  public ResultData updatePay(@RequestBody PayDTO payDTO);

  @DeleteMapping(value = "/pay/del/{id}")
  public ResultData deletePay(@PathVariable("id") Integer id);

  @GetMapping(value = "/pay/get/{id}")
  public ResultData getPay(@PathVariable("id") Integer id);

  @GetMapping(value = "/pay/get/info")
  public String mylb();
}
