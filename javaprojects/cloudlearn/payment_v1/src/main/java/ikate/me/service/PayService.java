package ikate.me.service;

import java.util.List;

import ikate.me.entities.Pay;

public interface PayService {
  int add(Pay pay);

  int delete(Integer id);

  int update(Pay pay);

  Pay getById(Integer id);

  List<Pay> getAll();
}
