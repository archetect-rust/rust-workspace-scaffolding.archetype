let context = #{};

const CASE_STRATEGY = CasedIdentityCasedValue(PROGRAMMING_CASES);

// This generally comes from ANSWERS, but prompting for it ensures it is in the context 
context += prompt("Project Author:", "author_full", #{
    placeholder: "Your Name <your.name@example.com>",
});

context += prompt("Project Name:", "project-name", #{
  cases: [
    CasedIdentityCasedValue(PROGRAMMING_CASES),
    FixedKeyCasedValue("project-title", TitleCase),
  ], 
});

context += prompt("Crates:", "crates", #{
  type: List,
});

render(Directory("contents/base"), context);

for crate in context.crates {
  let crate_context = context + set("crate-name", crate, #{
    cases: [
      CASE_STRATEGY,
      FixedKeyCasedValue("crate-title", TitleCase),
    ],
  });

  if crate == "bin" {
    render(Directory("contents/crate-bin"), crate_context);
  } else if crate == "xtask" {
    render(Directory("contents/crate-xtask"), crate_context);
  } else {
    render(Directory("contents/crate-lib"), crate_context);
  }

}

