## Atompunk style badge

Currently only supports the circleci.

### Example

```bash
git clone https://github.com/yetone/atompunk-badge.git && cd atompunk-badge

cargo run

open http://localhost:8080/{vcs-type}/{username}/{project}?token={token}
```

### Badges

#### SUCCESS

![](https://github.com/yetone/atompunk-badge/raw/master/assets/success.gif)

#### FAILED

![](https://github.com/yetone/atompunk-badge/raw/master/assets/failed.gif)

#### RUNNING

![](https://github.com/yetone/atompunk-badge/raw/master/assets/running.gif)

#### OH HOLD

![](https://github.com/yetone/atompunk-badge/raw/master/assets/oh_hold.gif)

#### QUEUED

![](https://github.com/yetone/atompunk-badge/raw/master/assets/queued.gif)
