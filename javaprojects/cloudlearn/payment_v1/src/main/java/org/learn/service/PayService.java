package org.learn.service;

import java.util.List;
import org.learn.entities.Pay;

public interface PayService {
  int add(Pay pay);

  int delete(Integer id);

  int update(Pay pay);

  Pay getById(Integer id);

  List<Pay> getAll();
}
