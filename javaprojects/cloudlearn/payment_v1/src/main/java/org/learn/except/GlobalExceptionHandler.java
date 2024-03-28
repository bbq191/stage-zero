package org.learn.except;

import lombok.extern.slf4j.Slf4j;
import org.learn.enums.ReturnCodeEnum;
import org.learn.resp.ResultData;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.ResponseStatus;
import org.springframework.web.bind.annotation.RestControllerAdvice;

@Slf4j
@RestControllerAdvice
public class GlobalExceptionHandler {
  /**
   * 默认全局异常处理。
   *
   * @param e the e
   * @return ResultData
   */
  @ExceptionHandler(RuntimeException.class)
  @ResponseStatus(HttpStatus.INTERNAL_SERVER_ERROR)
  public ResultData<String> exception(Exception e) {
    System.out.println("----come in GlobalExceptionHandler");
    log.error("全局异常信息exception:{}", e.getMessage(), e);
    return ResultData.fail(ReturnCodeEnum.RC500.getCode(), e.getMessage());
  }
}
