✨ Spark Quick Reference ✨

1. File Structure:
   ~forge~ = calm|balanced|wild
   ~weave~ = <number>
   @seeds@ ... @seeds@
   @spells@ ... @spells@

2. Method Calls:
   object**method()
   object**method1()**method2()

3. Configurations:
   settings: [
       key: value,
       nested: [
           items: [...],
       ]
   ]

4. Comments:
   >>> Single line comment
   >>> Multi-line
   >>> comments

5. Error Handling:
   try! [...] catch [err] [...]

6. Parallel Operations:
   data.parallel_map(|x| [...])
