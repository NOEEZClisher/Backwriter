# Backwriter 0.3.0 Version-Up Note

## Independent Namespace, Complete View & Verification Cleanup

> **목표**
>
> Backwriter의 private 저장 경로를 `.artext/bw/`에서 `.bw/`로 독립시키고, shell View가 실제 본문까지 반환하도록 완성한다.
>
> 동시에 도움말, 코드 배치, 현재 검증 문서를 정리한다.
>
> **Search 알고리즘과 Anddress v5의 의미는 변경하지 않는다.**

---

# BOX 1 — 0.3.0의 의미

0.3.0은 새로운 검색·편집 엔진을 만드는 버전이 아니다.

```text
0.2.x
구조 주소와 실행 모델 확립
CLI 사용 경로 정비

0.3.0
독립 제품의 private namespace 확정
shell 관찰 결과 완성
도움말·코드·검증 문서 정리
```

저장 경로 변경이 포함되므로 단순한 0.2.x 후속 패치와 구분한다.

핵심 원칙:

> **Backwriter가 자기 저장 공간을 소유하고, View가 사용자가 요청한 내용을 실제로 보여준다.**

---

# BOX 2 — 변경 범위

| 영역 | 0.3.0 변경 |
|---|---|
| Private 저장 경로 | `.artext/bw/` → `.bw/` |
| Direct shell View | projected ref와 실제 Content를 함께 출력 |
| Batch View 표시 | 입력별 ref·본문·RelationAbsent 대응을 명확히 표시 |
| Help | 실제 인자·작동·출력 중심으로 보강 |
| Advanced help | 사용 가능한 raw Session 문법을 발견할 수 있게 설명 |
| CLI 코드 배치 | 기존 구현을 책임별 내부 모듈로 분리 |
| 검증 문서 | 현재 계약과 현재 증거만 남기고 과거 기록은 링크 |
| Release 검증 | 0.2.6과 0.3.0만 active 비교 |

다음은 변경하지 않는다.

```text
Search matching / ranking / traversal algorithm
Search result ordering / multiplicity
Search JSON schema
Anddress v5 identity / canonical wire
currentness 판정
Apply publication 의미
Anchor 의미
history / relocation / automatic retry 정책
```

---

# BOX 3 — 저장 경로의 기준 위치는 유지

이번 변경은 **저장 디렉터리의 이름과 소유 namespace 변경**이다.

```text
BEFORE
<기존 저장 기준 위치>/.artext/bw/

AFTER
<동일한 저장 기준 위치>/.bw/
```

저장 기준 위치를 임의로 바꾸지 않는다.

```text
기존 시스템 경로 → workspace 내부로 이동 금지
기존 경로 → 사용자 홈으로 임의 이동 금지
새로운 환경변수·우선순위 체계 추가 금지
.bw/bw 같은 불필요한 중첩 금지
```

착수 시 실제 경로 결정 코드를 확인하고, **기존 기준 위치를 보존한 채 상대 경로만 변경**한다.

---

# BOX 4 — Hard Cut과 기존 데이터 처리

0.3.0은 새 private state를 `.bw/`에서만 읽고 쓴다.

```text
새 경로 읽기·쓰기: .bw/
구 경로 복원·fallback: 없음
자동 migration: 없음
구 경로 자동 삭제: 없음
```

기존 `.artext/bw/`가 있어도 새 경로로 복사하거나 복구하지 않는다. 필요한 재생성 가능한 상태는 현재 Source에서 다시 만든다.

단, 경로 내부에 **재생성 불가능한 사용자 데이터가 실제로 존재하는지**는 변경 전에 확인한다. 그런 데이터가 발견되면 private cache로 간주해 조용히 버리는 방식은 허용하지 않는다.

다른 Artext 데이터가 들어 있는 `.artext/` 전체를 삭제하거나 변경하지 않는다.

---

# BOX 5 — Private 경로 제외 규칙

`.bw/` 내부 자료가 일반 Workspace Source로 검색·편집되면 안 된다.

새 경로가 Source 후보 범위 안에 들어오는 경우:

```text
일반 Source 검색 대상에서 제외
일반 Source 직접 접근에서도 기존 private-path 정책 적용
임시 자료를 사용자 파일로 해석하지 않음
```

구 `.artext/bw/` 잔여물도 필요하다면 **최소한의 private-path 제외 규칙**으로 보호한다. 이는 구 상태를 읽거나 지원하는 compatibility path가 아니다.

경로 검사는 정확한 디렉터리 component를 기준으로 한다.

```text
.bw/          → Backwriter private 경로
.bw-notes/    → 이름이 비슷하다는 이유만으로 제외하지 않음
```

이 작업에 필요한 공통 경로 필터 수정은 허용한다. **Search 알고리즘 변경과는 구분한다.**

---

# BOX 6 — 경로 변경 감사 대상

단순 문자열 치환으로 끝내지 않는다.

다음 사용 지점을 확인한다.

```text
private root 생성·해석
state 읽기·쓰기
temporary/spill 경로
cleanup 대상
Source 제외 규칙
installer/update가 해당 경로를 사용하는지
테스트 fixture와 기대 경로
README·현재 architecture·help의 경로 안내
```

기존 경로의 권한·symlink·ordinary-directory 검증을 약화하지 않는다.

Help나 version처럼 private state가 필요 없는 명령은 `.bw/`를 불필요하게 생성하지 않는다.

과거 release 보고서에 기록된 `.artext/bw/`는 당시 사실이므로 일괄 치환하지 않는다.

---

# BOX 7 — 디스크 경로와 v5 식별자를 구분

이번 변경에서 가장 중요한 경계:

> **`.artext/bw/`라는 디스크 경로 변경은 Anddress wire version 변경이 아니다.**

다음은 그대로 유지한다.

```text
artext.backwriter-anddress.v5
canonical v5 field names
canonical encoding
hash transcript domain
workspace coordinate 산출 의미
source identity
```

`artext`라는 문자열이 보인다는 이유로 모두 `backwriter`나 `bw`로 바꾸지 않는다.

Wire 식별자·hash domain까지 변경하면 저장 경로 정리를 넘어 주소 호환성과 identity를 바꾸게 된다. 이는 0.3.0 범위 밖이다.

---

# BOX 8 — Shell View의 완성된 출력 계약

현재 필요한 수정은 단순하다.

```text
BEFORE
View
→ projected Anddress
→ 새 @ref
→ 위치만 출력

AFTER
View
→ projected Anddress + Content
→ 새 @ref
→ 위치와 실제 Content를 함께 출력
```

다음 명령 한 번으로 주변 Paragraph를 읽고 판단할 수 있어야 한다.

```text
view @0 @1 @2 --as paragraph
```

다음 우회는 더 이상 필요하지 않아야 한다.

```text
direct View
→ projected ref 다시 View
→ named binding 복제
→ raw View로 본문 읽기
```

---

# BOX 9 — Batch View는 입력별 완결된 결과를 출력

각 입력에 대응하는 정보를 한 덩어리로 출력한다.

필수 정보:

```text
어떤 input ref의 결과인가
새 projected ref는 무엇인가
target kind와 위치
실제 Content
```

표시 예:

```text
input @0 → @8  Paragraph  unit-01.txt:1-3
<해당 Paragraph의 실제 본문>

input @1 → @9  Paragraph  unit-01.txt:5-7
<해당 Paragraph의 실제 본문>

input @2 → RelationAbsent
```

입력 순서와 duplicate를 유지한다.

`RelationAbsent`만 마지막에 모아서 출력하지 않는다. 중간 입력에서 관계가 없었다면 **그 입력 위치에서** 표시한다.

`RelationAbsent`는 새로운 ref를 만들지 않는다.

---

# BOX 10 — 본문과 표시용 구분자를 구별

Content에는 다음이 있을 수 있다.

```text
LF
CR
CRLF
마지막 terminator 없음
빈 Content
출력 구분자와 비슷한 문자열
```

Shell의 제목·구분선·가독성을 위한 개행은 **표시용 metadata**다. 원래 Content에 포함된 바이트인 것처럼 설명하면 안 된다.

출력 형식을 확정할 때 본문 경계를 식별할 수 있게 하고, 마지막 개행 유무를 숨기지 않는다. 필요하면 byte length나 no-EOL 표시를 사용하되 새로운 주소 형식은 만들지 않는다.

기존 one-shot `--raw`와 JSON View의 exact Content 계약은 그대로 유지한다.

---

# BOX 11 — View는 이미 받은 Content를 출력

Shell writer는 Runtime이 반환한 다음 결과를 그대로 사용한다.

```text
ViewOutcome::Projected {
    anddress,
    content
}
```

금지:

```text
본문을 출력하려고 두 번째 Runtime View 호출
projected ref를 다시 resolve해 source 재관찰
별도 Search 실행
Content를 다른 parser로 재해석
동일 Content의 불필요한 추가 collection 생성
```

Batch는 기존 `view_batch()`를 사용한다. Writer가 batch를 개별 View 호출로 풀어버리지 않는다.

완료된 Runtime 결과를 한 번 순회하면서 ref와 Content를 출력하는 방향을 기본으로 한다.

---

# BOX 12 — Ref와 실패 계약 유지

기존 session ref 의미는 변경하지 않는다.

```text
process-local
append-only slot
slot 재사용 없음
silent rebinding 없음
shell 종료 시 소멸
disk persistence 없음
```

다른 바이트 상태로 변경된 Source의 이전 ordinary Anddress는 stale이다. 새 receipt는 별도의 ref로 반환한다.

같은 파일의 다른 위치를 수정해도 이전 주소가 stale일 수 있다는 사실을 help에서 설명한다. **“Search 한 번이면 같은 파일의 모든 주소를 계속 편집할 수 있다”는 의미로 안내하지 않는다.**

Runtime batch 실패 시 성공 결과를 꾸며 출력하지 않는다. 출력 도중 stdout이 실패하는 경우도 별도로 오류 처리하되, 이미 이루어진 publication을 취소했다고 주장하거나 자동 재시도하지 않는다.

---

# BOX 13 — Top-Level Help에서 Shell의 역할 수정

Shell을 고급 raw 명령 전용처럼 소개하지 않는다.

권장 설명:

```text
shell  Reuse short references across search, view, replace, and check.
```

Top-level help의 역할:

```text
실행 형태
주요 명령 목록
필수 global option 안내
개별 help를 찾는 방법
```

모든 세부 문법과 개념을 한 화면에 넣지 않는다.

`bw version`처럼 단순한 명령에 빈 ARGUMENTS·OPTIONS·WHAT HAPPENS 절을 반복해서 붙일 필요도 없다.

**공통 형식보다 필요한 정보를 빠르게 찾을 수 있는지가 우선이다.**

---

# BOX 14 — Shell Help는 Direct 문법을 직접 설명

`bw help shell`에서 다음 문법을 바로 찾을 수 있어야 한다.

```text
search <기존 Search 인자>

view <REF>... [--as <line|paragraph|file>]

replace <REF> <CONTENT>

check <REF>...

let <NAME> = <REF>

exit
```

반드시 설명할 것:

```text
@N과 named binding의 실제 표기
공백이 있는 문자열의 quoting
View가 Content와 새 ref를 함께 반환한다는 점
Line Replace에는 body만 전달한다는 점
Check가 Current 입력에 새 ref를 발행하는 규칙
수정 후 이전 same-source ref가 stale이 될 수 있다는 점
```

예시는 실제 parser에 맞춰 실행 검증한다. 숫자 ref를 고정한 예시는 그 번호가 생성되는 앞선 명령까지 포함한다.

---

# BOX 15 — Advanced Help의 막힌 안내 제거

다음과 같은 안내만으로 끝내지 않는다.

```text
Pick은 one-shot이 없으니 shell을 사용하라.
```

사용자가 `bw help pick`을 요청하면 **shell에서 사용하는 실제 Pick 문법**을 설명한다.

Anchor, Apply, Data도 같은 원칙을 적용한다.

```text
어느 실행 표면에서 사용할 수 있는가
필수 operand와 개수
binding/reference 표기
최소 실행 예시
출력과 실패 조건
```

이는 도움말 topic 추가다. one-shot Pick·Anchor·Apply·Data 실행 기능을 새로 만드는 작업이 아니다.

일반 workflow 설명과 advanced raw Session 설명은 분리한다.

---

# BOX 16 — Edit와 Content Transport는 유지

0.2.6에서 정리한 의미를 그대로 유지한다.

```text
일반 Line Edit / direct Replace
  body-only
  기존 None/LF/CR/CRLF 보존
  입력 NUL/CR/LF 거절

File / Paragraph Replace
  기존 exact Content 의미 유지

advanced raw Edit/Apply
  caller-owned exact extent
```

기존 one-shot `--stdin`도 유지한다.

Shell stdin은 명령 입력에 사용되므로, 이번에 direct shell Replace에 EOF 기반 `--stdin`을 무리하게 추가하지 않는다.

새로운 Edit executor, terminator 보정기, Content schema를 만들지 않는다.

---

# BOX 17 — CLI 물리적 모듈 분리

기존 단일 CLI 파일을 책임별 내부 모듈로 나눈다.

예:

```text
src/bin/bw.rs
src/bin/bw/help.rs
src/bin/bw/shell.rs
src/bin/bw/output.rs
src/bin/bw/error.rs
```

역할:

```text
bw.rs       entrypoint / top-level dispatch
help.rs     도움말과 공통 usage
shell.rs    direct refs와 기존 Session 처리
output.rs   human / JSON / raw / shell 출력
error.rs    오류 형식과 exit mapping
```

기존 함수와 타입의 위치를 옮기는 것이 기본이다.

```text
두 번째 parser 금지
Shell 전용 Runtime executor 금지
새 public crate 불필요
불필요한 trait·factory·wrapper 추가 금지
```

파일 분리만으로 실행 성능이나 컴파일 시간이 빨라졌다고 주장하지 않는다.

---

# BOX 18 — CLI 테스트도 하나의 Integration Crate 유지

테스트 파일은 필요하면 다음처럼 나눈다.

```text
tests/cli.rs
tests/cli/help.rs
tests/cli/edit.rs
tests/cli/view.rs
tests/cli/check.rs
tests/cli/shell.rs
```

`tests/cli.rs`가 내부 모듈을 포함하는 형태로 유지한다.

단순 정리를 위해 별도 integration binary를 여러 개 만들지 않는다.

공통 fixture와 setup은 재사용하되, 서로 다른 실패 경계의 검증을 삭제하지 않는다. 파일 크기를 줄이기 위해 coverage를 줄이는 것은 목표가 아니다.

---

# BOX 19 — `verification.md`를 실제로 축소

현재 검증 문서에는 다음만 남긴다.

```text
현재 검증 정책
현재 Source Authority
직전 비교 기준
현재 필수 correctness / target matrix
이번 release 결과 요약
과거 증거를 찾는 링크
```

과거 release별 상세 표·raw samples·단계별 서술은 현재 문서에서 제거한다.

```text
현재 문서
→ 현재 계약과 이번 결과

과거 task 문서
→ 해당 시점의 고정 증거
```

단, 오래된 절 안에 **현재도 유효한 공통 검증 규칙**이 있으면 짧은 현재 규칙으로 옮긴 뒤 중복 설명을 제거한다. 과거 기록이라는 이유만으로 현재 안전성 계약까지 삭제하지 않는다.

---

# BOX 20 — 과거 증거는 삭제가 아니라 분리

과거 증거가 기존 task 문서에 이미 있으면 링크만 남긴다.

현재 문서에만 존재하는 고유 증거는 보존 위치를 확보한 뒤 옮긴다.

```text
과거 benchmark 숫자 수정 없음
과거 측정 환경 변경 없음
과거 경로를 .bw로 소급 수정하지 않음
과거 Source Authority 재작성 없음
```

현재 `verification.md`에 모든 release 링크를 계속 추가할 필요도 없다.

```text
현재 N 링크
직전 N-1 링크
Historical evidence index 링크
```

정도로 유지해 active 문서가 다시 역사책으로 커지지 않게 한다.

README와 current 문서의 production-equivalent SHA도 실제 tree 비교 결과에 맞춰 정리한다. 이전 버전의 SHA를 복사해 넣지 않는다.

---

# BOX 21 — Active 비교는 0.2.6 ↔ 0.3.0

0.3.0의 직접 비교 기준:

```text
N-1:
Backwriter 0.2.6
Source Authority:
09bb6c424081594bd86a95f04345b786ef9b46b6

N:
Backwriter 0.3.0 candidate
실제 readiness 시점에 SHA 확정
```

실행하지 않는 것:

```text
0.2.5 이하 checkout / build / benchmark
과거 Search performance matrix
과거 release 전체 재검증
```

0.2.6의 기존 보고서는 참고 증거로 읽을 수 있다. 실제 개선율은 가능한 한 같은 조건에서 새로 실행한 N-1/N으로 계산한다.

---

# BOX 22 — 검증 반복 비용도 제한

개발 중에는 변경 영역의 focused tests를 실행한다.

```text
경로 정책
shell View writer
help
모듈 분리 후 연결
```

최종 candidate에서 현재 GNU/musl 전체 suite를 실행한다. 각 작은 Gate마다 두 target의 전체 suite를 반복하는 것을 기본 절차로 만들지 않는다.

검증 재사용 여부는 `src/**`만 보고 결정하지 않는다.

```text
production source
tests와 fixture
build script
Cargo.toml / Cargo.lock
toolchain
target / features / profile / 관련 build flags
```

변경 여부를 함께 확인한다.

문서만 바뀌면 기존 실행 증거를 재사용한다. Package version이나 build metadata가 바뀌면 binary identity와 Version KAT를 다시 확인하되, 그 변경만으로 모든 과거 벤치를 다시 실행하지 않는다.

---

# BOX 23 — Namespace Correctness

기존 경로·Source fixture를 이용해 최소한 다음을 검사한다.

| 조건 | 기대 결과 |
|---|---|
| 새 경로가 없음 | 필요한 명령이 기존 기준 위치 아래 `.bw/` 사용 |
| 구 `.artext/bw/`만 존재 | 구 상태를 읽거나 옮기거나 삭제하지 않음 |
| 구·신 경로가 함께 존재 | 새 private state는 `.bw/`만 사용 |
| 구 경로에 sentinel 파일 존재 | 모든 실행 뒤 sentinel byte-identical |
| 새 경로가 비정상 파일·symlink | 기존 안전한 경로 접근 규칙대로 거절 |
| `.bw/`가 Source 후보 안에 존재 | private 자료를 일반 Source로 노출하지 않음 |
| `.bw-notes/` 같은 유사 이름 | 과도한 문자열 매칭으로 제외하지 않음 |
| help/version만 실행 | 불필요한 private 디렉터리 생성 없음 |

새 저장 경로가 **Source 전체를 RAM에 올리거나**, 과거 Source 내용을 보존하는 근거가 되어서는 안 된다.

---

# BOX 24 — Shell View Correctness

다음은 AI 평가 이전에 deterministic test로 고정한다.

```text
single self View:
  ref + 실제 Content

Line → Paragraph:
  parent ref + 실제 Paragraph Content

Line → File:
  File ref + 실제 File Content

batch:
  input order와 duplicate 보존
  각 Content와 ref의 대응 보존

RelationAbsent:
  해당 입력 위치에 표시
  ref 발행 없음

None/LF/CR/CRLF:
  본문과 표시용 구분자를 혼동하지 않음

failure:
  기존 Runtime 실패 계약 유지
  불필요한 재조회 없음
```

핵심 구조 검사:

```text
한 direct single View → 한 Runtime View
한 direct batch View → 한 Runtime batch View
본문 출력용 추가 Search/View → 0
```

---

# BOX 25 — Dummy / Genie 평가

이번에도 외부 도구 비교는 하지 않는다.

```text
0.2.6 Dummy ↔ 0.3.0 Dummy
0.2.6 Genie ↔ 0.3.0 Genie
```

0.2.6 독립 벤치의 동일한 four-file fixture를 우선 재사용한다.

```text
8 duplicate Lines
primary 4개만 수정
파일당 수정 1개
LF / CR / CRLF / None
secondary 4개 보존
동일한 독립 full-byte oracle
```

Dummy는 공개 help만 보고 경로를 선택한다. one-shot을 선택했다는 이유만으로 실패 처리하거나 shell을 강제하지 않는다.

Genie는 공개 문서의 권장 shell 경로를 사용한다.

```text
Search 1
문맥용 batch Paragraph View 1
Replace 4
batch Check 1
최종 batch File View 1
```

총 **8개 capability 명령**을 기준 흐름으로 삼는다. Help, shell 시작·종료, 모델 도구 왕복 수와는 별도로 센다.

같은 파일에 여러 편집이 있는 다른 과제까지 Search 한 번으로 충분하다고 일반화하지 않는다.

---

# BOX 26 — AI Acceptance와 지표

가장 중요한 acceptance:

```text
첫 batch Paragraph View에서 primary를 판별할 본문 확보
본문을 얻으려는 반복 self View = 0
본문을 얻으려는 named-binding 복제 = 0
본문을 얻으려는 raw View 우회 = 0
terminator 실수 = 0
Wrong Apply = 0
최종 oracle exact-match
```

다음 지표는 분리해서 기록한다.

```text
bw process 수
실제 bw 명령 수
모델 도구 왕복 수
예상 밖 CLI 실패 수
본문 획득을 위한 추가 명령 수
bw stdout/stderr bytes
모델 가시 tool-output bytes
elapsed
```

0.2.6에서 누락했던 본문을 새로 출력하므로, **출력 byte 수가 증가했다는 이유만으로 회귀로 판정하지 않는다.**

목표는 내용을 덜 보여주는 것이 아니라:

> **필요한 내용을 한 응답에서 제공해 추가 조회와 판단 왕복을 줄이는 것.**

각 AI arm이 n=1이면 시간·왕복 감소율은 관찰값이다. 고정 비율 달성 여부만으로 실제 correctness와 기능 완성도를 뒤집지 않는다.

---

# BOX 27 — 유지할 안전성 경계와 범위 밖 작업

기존 계약은 그대로 유지한다.

```text
pre-existing stale state는 fail-close
ordinary address relocation 없음
automatic retry 없음
history 없음
merge 없음
shell ref의 disk persistence 없음
```

`.bw/` 도입은 history 저장이나 세션 복구 기능 도입이 아니다.

이번에 추가하지 않는 것:

```text
multi-file transaction
rollback
concurrent-writer CAS
filesystem locking 설계
Search wire compression
NDJSON Search schema
source identity interning
musl throughput 최적화
새 reference lifecycle
```

여러 Replace는 여전히 여러 번의 publication이다. 저장 namespace나 shell 출력 변경이 기존보다 강한 atomicity를 제공한다고 설명하지 않는다.

---

# BOX 28 — 실행 순서와 최종 판정

권장 작업 순서:

```text
Gate 1
실제 private 경로 사용처 확인
.bw hard-cut 및 v5 비변경 경계 고정

Gate 2
private namespace 변경
경로·제외 규칙 focused tests

Gate 3
direct shell View의 ref + Content 출력
batch 순서·본문 경계 focused tests

Gate 4
help 보강
CLI·테스트 내부 모듈 분리
verification 현재 문서 축소

Gate 5
현재 candidate 최종 GNU/musl suite
0.2.6 ↔ 0.3.0 Dummy/Genie
artifact identity와 release closure
```

최종 GO 조건:

| 영역 | 완료 기준 |
|---|---|
| 독립 namespace | 기존 기준 위치의 `.bw/`만 새 private state에 사용 |
| 구 상태 보호 | 자동 migration·삭제 없음, 구 자료 일반 Source 유입 없음 |
| v5 | wire·identity·hash domain 유지 |
| Shell View | ref와 본문을 한 번에 반환, 재조회 우회 불필요 |
| Help | direct/advanced 문법과 실제 출력 설명이 구현과 일치 |
| 코드 조직 | 내부 모듈 분리, parser·executor 중복 없음 |
| 검증 문서 | 현재 계약·현재 증거·historical 링크만 유지 |
| 검증 비용 | N-1/N만 실행, Gate별 불필요한 full-suite 반복 없음 |
| 정확성 | exact bytes·순서·중복·fresh/stale 계약 유지 |

## 최종 정의

> **0.3.0은 Backwriter가 자기 저장 공간을 갖고, View가 실제 내용을 보여주며, 현재 개발 작업이 과거 검증 기록에 묻히지 않게 만드는 버전이다.**

**Own namespace. Complete observations. Bounded verification.**
