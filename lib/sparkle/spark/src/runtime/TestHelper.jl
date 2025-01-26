module TestHelper
    using Test
    export @spark_test

    macro spark_test(expr)
        quote
            @testset "Spark Tests" begin
                $(esc(expr))
            end
        end
    end
end
