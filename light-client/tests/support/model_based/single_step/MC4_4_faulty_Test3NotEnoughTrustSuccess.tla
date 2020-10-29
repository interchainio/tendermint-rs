------------------------- MODULE counterexample -------------------------

EXTENDS MC4_4_faulty

(* Initial state *)

State1 ==
TRUE
(* Transition 0 to State2 *)

State2 ==
/\ Faulty = {}
/\ blockchain = 1
    :> [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]
  @@ 2
    :> [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]
  @@ 3
    :> [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]
  @@ 4
    :> [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]
  @@ 5
    :> [NextVS |-> { "n1", "n2", "n3", "n4" },
      VS |-> {"n4"},
      height |-> 5,
      lastCommit |-> {"n2"},
      time |-> 11]
/\ fetchedLightBlocks = 1
    :> [Commits |-> { "n1", "n2", "n3", "n4" },
      header |->
        [NextVS |-> { "n1", "n2", "n3" },
          VS |-> { "n1", "n2", "n3", "n4" },
          height |-> 1,
          lastCommit |-> {},
          time |-> 1]]
/\ history = 0
    :> [current |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]],
      now |-> 11,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
/\ latestVerified = [Commits |-> { "n1", "n2", "n3", "n4" },
  header |->
    [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]]
/\ lightBlockStatus = 1 :> "StateVerified"
/\ nextHeight = 4
/\ now = 11
/\ nprobes = 0
/\ prevCurrent = [Commits |-> { "n1", "n2", "n3", "n4" },
  header |->
    [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]]
/\ prevNow = 11
/\ prevVerdict = "SUCCESS"
/\ prevVerified = [Commits |-> { "n1", "n2", "n3", "n4" },
  header |->
    [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]]
/\ state = "working"

(* Transition 1 to State3 *)

State3 ==
/\ Faulty = {}
/\ blockchain = 1
    :> [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]
  @@ 2
    :> [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]
  @@ 3
    :> [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]
  @@ 4
    :> [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]
  @@ 5
    :> [NextVS |-> { "n1", "n2", "n3", "n4" },
      VS |-> {"n4"},
      height |-> 5,
      lastCommit |-> {"n2"},
      time |-> 11]
/\ fetchedLightBlocks = 1
    :> [Commits |-> { "n1", "n2", "n3", "n4" },
      header |->
        [NextVS |-> { "n1", "n2", "n3" },
          VS |-> { "n1", "n2", "n3", "n4" },
          height |-> 1,
          lastCommit |-> {},
          time |-> 1]]
  @@ 4
    :> [Commits |-> {"n2"},
      header |->
        [NextVS |-> {"n4"},
          VS |-> {"n2"},
          height |-> 4,
          lastCommit |-> {"n3"},
          time |-> 10]]
/\ history = 0
    :> [current |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]],
      now |-> 11,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 1
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 11,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
/\ latestVerified = [Commits |-> { "n1", "n2", "n3", "n4" },
  header |->
    [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]]
/\ lightBlockStatus = 1 :> "StateVerified" @@ 4 :> "StateUnverified"
/\ nextHeight = 3
/\ now = 12
/\ nprobes = 1
/\ prevCurrent = [Commits |-> {"n2"},
  header |->
    [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]]
/\ prevNow = 11
/\ prevVerdict = "NOT_ENOUGH_TRUST"
/\ prevVerified = [Commits |-> { "n1", "n2", "n3", "n4" },
  header |->
    [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]]
/\ state = "working"

(* Transition 1 to State4 *)

State4 ==
/\ Faulty = {}
/\ blockchain = 1
    :> [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]
  @@ 2
    :> [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]
  @@ 3
    :> [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]
  @@ 4
    :> [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]
  @@ 5
    :> [NextVS |-> { "n1", "n2", "n3", "n4" },
      VS |-> {"n4"},
      height |-> 5,
      lastCommit |-> {"n2"},
      time |-> 11]
/\ fetchedLightBlocks = 1
    :> [Commits |-> { "n1", "n2", "n3", "n4" },
      header |->
        [NextVS |-> { "n1", "n2", "n3" },
          VS |-> { "n1", "n2", "n3", "n4" },
          height |-> 1,
          lastCommit |-> {},
          time |-> 1]]
  @@ 3
    :> [Commits |-> {"n3"},
      header |->
        [NextVS |-> {"n2"},
          VS |-> {"n3"},
          height |-> 3,
          lastCommit |-> { "n1", "n2", "n3" },
          time |-> 9]]
  @@ 4
    :> [Commits |-> {"n2"},
      header |->
        [NextVS |-> {"n4"},
          VS |-> {"n2"},
          height |-> 4,
          lastCommit |-> {"n3"},
          time |-> 10]]
/\ history = 0
    :> [current |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]],
      now |-> 11,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 1
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 11,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 2
    :> [current |->
        [Commits |-> {"n3"},
          header |->
            [NextVS |-> {"n2"},
              VS |-> {"n3"},
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 9]],
      now |-> 12,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
/\ latestVerified = [Commits |-> { "n1", "n2", "n3", "n4" },
  header |->
    [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]]
/\ lightBlockStatus = 1 :> "StateVerified" @@ 3 :> "StateUnverified" @@ 4 :> "StateUnverified"
/\ nextHeight = 2
/\ now = 12
/\ nprobes = 2
/\ prevCurrent = [Commits |-> {"n3"},
  header |->
    [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]]
/\ prevNow = 12
/\ prevVerdict = "NOT_ENOUGH_TRUST"
/\ prevVerified = [Commits |-> { "n1", "n2", "n3", "n4" },
  header |->
    [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]]
/\ state = "working"

(* Transition 3 to State5 *)

State5 ==
/\ Faulty = {}
/\ blockchain = 1
    :> [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]
  @@ 2
    :> [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]
  @@ 3
    :> [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]
  @@ 4
    :> [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]
  @@ 5
    :> [NextVS |-> { "n1", "n2", "n3", "n4" },
      VS |-> {"n4"},
      height |-> 5,
      lastCommit |-> {"n2"},
      time |-> 11]
/\ fetchedLightBlocks = 1
    :> [Commits |-> { "n1", "n2", "n3", "n4" },
      header |->
        [NextVS |-> { "n1", "n2", "n3" },
          VS |-> { "n1", "n2", "n3", "n4" },
          height |-> 1,
          lastCommit |-> {},
          time |-> 1]]
  @@ 2
    :> [Commits |-> { "n1", "n2", "n3" },
      header |->
        [NextVS |-> {"n3"},
          VS |-> { "n1", "n2", "n3" },
          height |-> 2,
          lastCommit |-> { "n1", "n2", "n3", "n4" },
          time |-> 2]]
  @@ 3
    :> [Commits |-> {"n3"},
      header |->
        [NextVS |-> {"n2"},
          VS |-> {"n3"},
          height |-> 3,
          lastCommit |-> { "n1", "n2", "n3" },
          time |-> 9]]
  @@ 4
    :> [Commits |-> {"n2"},
      header |->
        [NextVS |-> {"n4"},
          VS |-> {"n2"},
          height |-> 4,
          lastCommit |-> {"n3"},
          time |-> 10]]
/\ history = 0
    :> [current |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]],
      now |-> 11,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 1
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 11,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 2
    :> [current |->
        [Commits |-> {"n3"},
          header |->
            [NextVS |-> {"n2"},
              VS |-> {"n3"},
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 9]],
      now |-> 12,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 3
    :> [current |->
        [Commits |-> { "n1", "n2", "n3" },
          header |->
            [NextVS |-> {"n3"},
              VS |-> { "n1", "n2", "n3" },
              height |-> 2,
              lastCommit |-> { "n1", "n2", "n3", "n4" },
              time |-> 2]],
      now |-> 12,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
/\ latestVerified = [Commits |-> { "n1", "n2", "n3" },
  header |->
    [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]]
/\ lightBlockStatus = 1 :> "StateVerified"
  @@ 2 :> "StateVerified"
  @@ 3 :> "StateUnverified"
  @@ 4 :> "StateUnverified"
/\ nextHeight = 4
/\ now = 12
/\ nprobes = 3
/\ prevCurrent = [Commits |-> { "n1", "n2", "n3" },
  header |->
    [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]]
/\ prevNow = 12
/\ prevVerdict = "SUCCESS"
/\ prevVerified = [Commits |-> { "n1", "n2", "n3", "n4" },
  header |->
    [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]]
/\ state = "working"

(* Transition 1 to State6 *)

State6 ==
/\ Faulty = {}
/\ blockchain = 1
    :> [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]
  @@ 2
    :> [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]
  @@ 3
    :> [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]
  @@ 4
    :> [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]
  @@ 5
    :> [NextVS |-> { "n1", "n2", "n3", "n4" },
      VS |-> {"n4"},
      height |-> 5,
      lastCommit |-> {"n2"},
      time |-> 11]
/\ fetchedLightBlocks = 1
    :> [Commits |-> { "n1", "n2", "n3", "n4" },
      header |->
        [NextVS |-> { "n1", "n2", "n3" },
          VS |-> { "n1", "n2", "n3", "n4" },
          height |-> 1,
          lastCommit |-> {},
          time |-> 1]]
  @@ 2
    :> [Commits |-> { "n1", "n2", "n3" },
      header |->
        [NextVS |-> {"n3"},
          VS |-> { "n1", "n2", "n3" },
          height |-> 2,
          lastCommit |-> { "n1", "n2", "n3", "n4" },
          time |-> 2]]
  @@ 3
    :> [Commits |-> {"n3"},
      header |->
        [NextVS |-> {"n2"},
          VS |-> {"n3"},
          height |-> 3,
          lastCommit |-> { "n1", "n2", "n3" },
          time |-> 9]]
  @@ 4
    :> [Commits |-> {"n2"},
      header |->
        [NextVS |-> {"n4"},
          VS |-> {"n2"},
          height |-> 4,
          lastCommit |-> {"n3"},
          time |-> 10]]
/\ history = 0
    :> [current |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]],
      now |-> 11,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 1
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 11,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 2
    :> [current |->
        [Commits |-> {"n3"},
          header |->
            [NextVS |-> {"n2"},
              VS |-> {"n3"},
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 9]],
      now |-> 12,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 3
    :> [current |->
        [Commits |-> { "n1", "n2", "n3" },
          header |->
            [NextVS |-> {"n3"},
              VS |-> { "n1", "n2", "n3" },
              height |-> 2,
              lastCommit |-> { "n1", "n2", "n3", "n4" },
              time |-> 2]],
      now |-> 12,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 4
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 12,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3" },
          header |->
            [NextVS |-> {"n3"},
              VS |-> { "n1", "n2", "n3" },
              height |-> 2,
              lastCommit |-> { "n1", "n2", "n3", "n4" },
              time |-> 2]]]
/\ latestVerified = [Commits |-> { "n1", "n2", "n3" },
  header |->
    [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]]
/\ lightBlockStatus = 1 :> "StateVerified"
  @@ 2 :> "StateVerified"
  @@ 3 :> "StateUnverified"
  @@ 4 :> "StateUnverified"
/\ nextHeight = 3
/\ now = 12
/\ nprobes = 4
/\ prevCurrent = [Commits |-> {"n2"},
  header |->
    [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]]
/\ prevNow = 12
/\ prevVerdict = "NOT_ENOUGH_TRUST"
/\ prevVerified = [Commits |-> { "n1", "n2", "n3" },
  header |->
    [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]]
/\ state = "working"

(* Transition 0 to State7 *)

State7 ==
/\ Faulty = {}
/\ blockchain = 1
    :> [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]
  @@ 2
    :> [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]
  @@ 3
    :> [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]
  @@ 4
    :> [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]
  @@ 5
    :> [NextVS |-> { "n1", "n2", "n3", "n4" },
      VS |-> {"n4"},
      height |-> 5,
      lastCommit |-> {"n2"},
      time |-> 11]
/\ fetchedLightBlocks = 1
    :> [Commits |-> { "n1", "n2", "n3", "n4" },
      header |->
        [NextVS |-> { "n1", "n2", "n3" },
          VS |-> { "n1", "n2", "n3", "n4" },
          height |-> 1,
          lastCommit |-> {},
          time |-> 1]]
  @@ 2
    :> [Commits |-> { "n1", "n2", "n3" },
      header |->
        [NextVS |-> {"n3"},
          VS |-> { "n1", "n2", "n3" },
          height |-> 2,
          lastCommit |-> { "n1", "n2", "n3", "n4" },
          time |-> 2]]
  @@ 3
    :> [Commits |-> {"n3"},
      header |->
        [NextVS |-> {"n2"},
          VS |-> {"n3"},
          height |-> 3,
          lastCommit |-> { "n1", "n2", "n3" },
          time |-> 9]]
  @@ 4
    :> [Commits |-> {"n2"},
      header |->
        [NextVS |-> {"n4"},
          VS |-> {"n2"},
          height |-> 4,
          lastCommit |-> {"n3"},
          time |-> 10]]
/\ history = 0
    :> [current |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]],
      now |-> 11,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 1
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 11,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 2
    :> [current |->
        [Commits |-> {"n3"},
          header |->
            [NextVS |-> {"n2"},
              VS |-> {"n3"},
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 9]],
      now |-> 12,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 3
    :> [current |->
        [Commits |-> { "n1", "n2", "n3" },
          header |->
            [NextVS |-> {"n3"},
              VS |-> { "n1", "n2", "n3" },
              height |-> 2,
              lastCommit |-> { "n1", "n2", "n3", "n4" },
              time |-> 2]],
      now |-> 12,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 4
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 12,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3" },
          header |->
            [NextVS |-> {"n3"},
              VS |-> { "n1", "n2", "n3" },
              height |-> 2,
              lastCommit |-> { "n1", "n2", "n3", "n4" },
              time |-> 2]]]
  @@ 5
    :> [current |->
        [Commits |-> {"n3"},
          header |->
            [NextVS |-> {"n2"},
              VS |-> {"n3"},
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 9]],
      now |-> 12,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3" },
          header |->
            [NextVS |-> {"n3"},
              VS |-> { "n1", "n2", "n3" },
              height |-> 2,
              lastCommit |-> { "n1", "n2", "n3", "n4" },
              time |-> 2]]]
/\ latestVerified = [Commits |-> {"n3"},
  header |->
    [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]]
/\ lightBlockStatus = 1 :> "StateVerified"
  @@ 2 :> "StateVerified"
  @@ 3 :> "StateVerified"
  @@ 4 :> "StateUnverified"
/\ nextHeight = 4
/\ now = 1408
/\ nprobes = 5
/\ prevCurrent = [Commits |-> {"n3"},
  header |->
    [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]]
/\ prevNow = 12
/\ prevVerdict = "SUCCESS"
/\ prevVerified = [Commits |-> { "n1", "n2", "n3" },
  header |->
    [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]]
/\ state = "working"

(* Transition 0 to State8 *)

State8 ==
/\ Faulty = {}
/\ blockchain = 1
    :> [NextVS |-> { "n1", "n2", "n3" },
      VS |-> { "n1", "n2", "n3", "n4" },
      height |-> 1,
      lastCommit |-> {},
      time |-> 1]
  @@ 2
    :> [NextVS |-> {"n3"},
      VS |-> { "n1", "n2", "n3" },
      height |-> 2,
      lastCommit |-> { "n1", "n2", "n3", "n4" },
      time |-> 2]
  @@ 3
    :> [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]
  @@ 4
    :> [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]
  @@ 5
    :> [NextVS |-> { "n1", "n2", "n3", "n4" },
      VS |-> {"n4"},
      height |-> 5,
      lastCommit |-> {"n2"},
      time |-> 11]
/\ fetchedLightBlocks = 1
    :> [Commits |-> { "n1", "n2", "n3", "n4" },
      header |->
        [NextVS |-> { "n1", "n2", "n3" },
          VS |-> { "n1", "n2", "n3", "n4" },
          height |-> 1,
          lastCommit |-> {},
          time |-> 1]]
  @@ 2
    :> [Commits |-> { "n1", "n2", "n3" },
      header |->
        [NextVS |-> {"n3"},
          VS |-> { "n1", "n2", "n3" },
          height |-> 2,
          lastCommit |-> { "n1", "n2", "n3", "n4" },
          time |-> 2]]
  @@ 3
    :> [Commits |-> {"n3"},
      header |->
        [NextVS |-> {"n2"},
          VS |-> {"n3"},
          height |-> 3,
          lastCommit |-> { "n1", "n2", "n3" },
          time |-> 9]]
  @@ 4
    :> [Commits |-> {"n2"},
      header |->
        [NextVS |-> {"n4"},
          VS |-> {"n2"},
          height |-> 4,
          lastCommit |-> {"n3"},
          time |-> 10]]
/\ history = 0
    :> [current |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]],
      now |-> 11,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 1
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 11,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 2
    :> [current |->
        [Commits |-> {"n3"},
          header |->
            [NextVS |-> {"n2"},
              VS |-> {"n3"},
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 9]],
      now |-> 12,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 3
    :> [current |->
        [Commits |-> { "n1", "n2", "n3" },
          header |->
            [NextVS |-> {"n3"},
              VS |-> { "n1", "n2", "n3" },
              height |-> 2,
              lastCommit |-> { "n1", "n2", "n3", "n4" },
              time |-> 2]],
      now |-> 12,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]]
  @@ 4
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 12,
      verdict |-> "NOT_ENOUGH_TRUST",
      verified |->
        [Commits |-> { "n1", "n2", "n3" },
          header |->
            [NextVS |-> {"n3"},
              VS |-> { "n1", "n2", "n3" },
              height |-> 2,
              lastCommit |-> { "n1", "n2", "n3", "n4" },
              time |-> 2]]]
  @@ 5
    :> [current |->
        [Commits |-> {"n3"},
          header |->
            [NextVS |-> {"n2"},
              VS |-> {"n3"},
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 9]],
      now |-> 12,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> { "n1", "n2", "n3" },
          header |->
            [NextVS |-> {"n3"},
              VS |-> { "n1", "n2", "n3" },
              height |-> 2,
              lastCommit |-> { "n1", "n2", "n3", "n4" },
              time |-> 2]]]
  @@ 6
    :> [current |->
        [Commits |-> {"n2"},
          header |->
            [NextVS |-> {"n4"},
              VS |-> {"n2"},
              height |-> 4,
              lastCommit |-> {"n3"},
              time |-> 10]],
      now |-> 1408,
      verdict |-> "SUCCESS",
      verified |->
        [Commits |-> {"n3"},
          header |->
            [NextVS |-> {"n2"},
              VS |-> {"n3"},
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 9]]]
/\ latestVerified = [Commits |-> {"n2"},
  header |->
    [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]]
/\ lightBlockStatus = 1 :> "StateVerified"
  @@ 2 :> "StateVerified"
  @@ 3 :> "StateVerified"
  @@ 4 :> "StateVerified"
/\ nextHeight = 4
/\ now = 1408
/\ nprobes = 6
/\ prevCurrent = [Commits |-> {"n2"},
  header |->
    [NextVS |-> {"n4"},
      VS |-> {"n2"},
      height |-> 4,
      lastCommit |-> {"n3"},
      time |-> 10]]
/\ prevNow = 1408
/\ prevVerdict = "SUCCESS"
/\ prevVerified = [Commits |-> {"n3"},
  header |->
    [NextVS |-> {"n2"},
      VS |-> {"n3"},
      height |-> 3,
      lastCommit |-> { "n1", "n2", "n3" },
      time |-> 9]]
/\ state = "finishedSuccess"

(* The following formula holds true in the last state and violates the invariant *)

InvariantViolation ==
  state = "finishedSuccess"
    /\ BMC!Skolem((\E s1$2 \in DOMAIN history:
      BMC!Skolem((\E s2$2 \in DOMAIN history:
        BMC!Skolem((\E s3$2 \in DOMAIN history:
          ((~(s1$2 = s2$2) /\ ~(s2$2 = s3$2)) /\ ~(s1$2 = s3$2))
            /\ history[s1$2]["verdict"] = "NOT_ENOUGH_TRUST"
            /\ history[s2$2]["verdict"] = "NOT_ENOUGH_TRUST"
            /\ history[s3$2]["verdict"] = "NOT_ENOUGH_TRUST"))))))

================================================================================
\* Created by Apalache on Thu Oct 29 14:03:06 CET 2020
\* https://github.com/informalsystems/apalache
