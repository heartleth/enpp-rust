<p align=center>
<img src="http://engplus.herokuapp.com/enpp.png" width=200/>
<h1 align=center>English++</h1>
<pre align=center><code>"hello" was print`ed</code></pre>
</p>
Eng++는 C++로 트랜스파일링되는 프로그래밍 언어입니다.
## 객체지향
English++ 은 객체지향과 동시에 함수혐 프로그래밍 언어입니다.

주어(객체)와 동사(멤버 변수)가 있고, 목적어 또는 보어(매개변수)가 있습니다

### 주어와 동사
능동태에서, 주어는 동사를 행할 수 있습니다.

- `It print` 는 C++에서는 `print()` 와 같습니다.  
- `vect clear` 는 C++에서는 `vect.clear()` 와 같습니다.

### 전치사, 보어/목적어
전치사의 종류는 다음이 있습니다. 어떤 상황에 어떤 전치사를 쓰던 동치이지만, 가독성을 위해 적절한 전치사를 사용하는 것이 좋습니다.
- with
- about
- for
- of
- to

이 외에도 `:`, `->` 가 있습니다. 
보어 또는 목적어를 쓰려면 전치사를 앞에 써 주어야 합니다.

```asm
It print about 10  ; 10에 대해 출력한다
It print with 10  ; 10이랑 같이 출력하다
```

기본적인 문법은 여기까지입니다.

### 목적격 관계대명사
```asm
It print about that it make_string of 10
; 10을 문자열로 만든 것을 출력하다
```

위의 평서문 문법은 결국에는 함수의 호출이지만, 함수는 값을 반환합니다. 이 값을 따오려면 목적격 관계 대명사의 `that`을 사용합니다.  
실제 영어처럼 생략은 되지 않습니다

### to 부정사
다른 언어로 치면 커링과 비슷하지만, 다른 의미도 있습니다.  
to 부정사는 english++만의 특성입니다. C++ 에서는 함수 또한 값으로 취급되기 때문에, 영작하면 이상합니다
```asm
    It call of print
    ; It의 call 동작은 없지만, 설명을 위해 썼습니다.
```
    
출력하다를 호출하다? eng++ 능동태 평서문에서 주어 it을 생략할수는 없지만, print함수도 값입니다.
이를 자연스럽게 하려면 동명사 `printing` 또는 to 부정사 `to print`가 필요한데, 여기서 to 부정사를 eng++에서 쓸 수 있습니다. to print 는 출력하기 라는 뜻으로, `출력하기를 호출하기`는 꽤 자연스럽습니다.

커링의 목적으로 사용할 수 있습니다.

```asm
to print about 10
```

는 10을 출력하는 새로운 함수를 만들어 냅니다.

### 수동태
```asm
"hello" was print`ed
```
Eng++에서 주어가 it인 경우에는 수동태의 by를 생략할 수 있습니다.  
만약 주어가 it이 아니라면, `-by-` 로 연결 해 주어야 합니다.
```asm
10 was push_back`ed-by-vect
; 이 경우 -to- 가 더 자연스럽지만 아직 지원을 하지 않습니다. 조만간 지원할 예정
```

정말 자연스럽죠?
