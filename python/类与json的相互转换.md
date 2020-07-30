```python
import json
class MyList:
    def __init__(self, l=None, name=None):
        self.l = l
        self.name = name

    @staticmethod
    def dict_to_object(d):
        return MyList(d.get('l'), d.get('name'))

    def to_json(self) -> str:
        return json.dumps(self, default=lambda obj: obj.__dict__,
                          ensure_ascii=False, sort_keys=True)
# 类->json
ml = MyList([1, 2, 3, 4], "wby")
ml = ml.to_json()
print(ml)

# json->类
ml = json.loads(ml, object_hook=MyList.dict_to_object)
print(ml)
print(ml.name)
print(ml.list)

```

