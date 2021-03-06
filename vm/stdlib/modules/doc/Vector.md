
<a name="0x1_Vector"></a>

# Module `0x1::Vector`



-  [Constants](#@Constants_0)
-  [Function `empty`](#0x1_Vector_empty)
-  [Function `length`](#0x1_Vector_length)
-  [Function `borrow`](#0x1_Vector_borrow)
-  [Function `push_back`](#0x1_Vector_push_back)
-  [Function `borrow_mut`](#0x1_Vector_borrow_mut)
-  [Function `pop_back`](#0x1_Vector_pop_back)
-  [Function `destroy_empty`](#0x1_Vector_destroy_empty)
-  [Function `swap`](#0x1_Vector_swap)
-  [Function `singleton`](#0x1_Vector_singleton)
-  [Function `reverse`](#0x1_Vector_reverse)
-  [Function `append`](#0x1_Vector_append)
-  [Function `is_empty`](#0x1_Vector_is_empty)
-  [Function `contains`](#0x1_Vector_contains)
-  [Function `index_of`](#0x1_Vector_index_of)
-  [Function `remove`](#0x1_Vector_remove)
-  [Function `swap_remove`](#0x1_Vector_swap_remove)
-  [Function `split`](#0x1_Vector_split)
-  [Specification](#@Specification_1)
    -  [Function `length`](#@Specification_1_length)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `singleton`](#@Specification_1_singleton)
    -  [Function `reverse`](#@Specification_1_reverse)
    -  [Function `append`](#@Specification_1_append)
    -  [Function `is_empty`](#@Specification_1_is_empty)
    -  [Function `contains`](#@Specification_1_contains)
    -  [Function `index_of`](#@Specification_1_index_of)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `swap_remove`](#@Specification_1_swap_remove)
    -  [Function `split`](#@Specification_1_split)
    -  [Module specifications](#@Module_specifications_2)


<pre><code></code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x1_Vector_EINDEX_OUT_OF_BOUNDS"></a>



<pre><code><b>const</b> <a href="Vector.md#0x1_Vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>: u64 = 0;
</code></pre>



<a name="0x1_Vector_empty"></a>

## Function `empty`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_empty">empty</a>&lt;Element&gt;(): vector&lt;Element&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_empty">empty</a>&lt;Element&gt;(): vector&lt;Element&gt;;
</code></pre>



</details>

<a name="0x1_Vector_length"></a>

## Function `length`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_length">length</a>&lt;Element&gt;(v: &vector&lt;Element&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_length">length</a>&lt;Element&gt;(v: &vector&lt;Element&gt;): u64;
</code></pre>



</details>

<a name="0x1_Vector_borrow"></a>

## Function `borrow`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_borrow">borrow</a>&lt;Element&gt;(v: &vector&lt;Element&gt;, i: u64): &Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_borrow">borrow</a>&lt;Element&gt;(v: &vector&lt;Element&gt;, i: u64): &Element;
</code></pre>



</details>

<a name="0x1_Vector_push_back"></a>

## Function `push_back`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_push_back">push_back</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, e: Element)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_push_back">push_back</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, e: Element);
</code></pre>



</details>

<a name="0x1_Vector_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_borrow_mut">borrow_mut</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, idx: u64): &<b>mut</b> Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_borrow_mut">borrow_mut</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, idx: u64): &<b>mut</b> Element;
</code></pre>



</details>

<a name="0x1_Vector_pop_back"></a>

## Function `pop_back`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_pop_back">pop_back</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;): Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_pop_back">pop_back</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;): Element;
</code></pre>



</details>

<a name="0x1_Vector_destroy_empty"></a>

## Function `destroy_empty`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_destroy_empty">destroy_empty</a>&lt;Element&gt;(v: vector&lt;Element&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_destroy_empty">destroy_empty</a>&lt;Element&gt;(v: vector&lt;Element&gt;);
</code></pre>



</details>

<a name="0x1_Vector_swap"></a>

## Function `swap`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_swap">swap</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, i: u64, j: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_swap">swap</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, i: u64, j: u64);
</code></pre>



</details>

<a name="0x1_Vector_singleton"></a>

## Function `singleton`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_singleton">singleton</a>&lt;Element&gt;(e: Element): vector&lt;Element&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_singleton">singleton</a>&lt;Element&gt;(e: Element): vector&lt;Element&gt; {
    <b>let</b> v = <a href="Vector.md#0x1_Vector_empty">empty</a>();
    <a href="Vector.md#0x1_Vector_push_back">push_back</a>(&<b>mut</b> v, e);
    v
}
</code></pre>



</details>

<a name="0x1_Vector_reverse"></a>

## Function `reverse`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_reverse">reverse</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_reverse">reverse</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;) {
    <b>let</b> len = <a href="Vector.md#0x1_Vector_length">length</a>(v);
    <b>if</b> (len == 0) <b>return</b> ();

    <b>let</b> front_index = 0;
    <b>let</b> back_index = len -1;
    <b>while</b> (front_index &lt; back_index) {
        <a href="Vector.md#0x1_Vector_swap">swap</a>(v, front_index, back_index);
        front_index = front_index + 1;
        back_index = back_index - 1;
    }
}
</code></pre>



</details>

<a name="0x1_Vector_append"></a>

## Function `append`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_append">append</a>&lt;Element&gt;(lhs: &<b>mut</b> vector&lt;Element&gt;, other: vector&lt;Element&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_append">append</a>&lt;Element&gt;(lhs: &<b>mut</b> vector&lt;Element&gt;, other: vector&lt;Element&gt;) {
    <a href="Vector.md#0x1_Vector_reverse">reverse</a>(&<b>mut</b> other);
    <b>while</b> (!<a href="Vector.md#0x1_Vector_is_empty">is_empty</a>(&other)) <a href="Vector.md#0x1_Vector_push_back">push_back</a>(lhs, <a href="Vector.md#0x1_Vector_pop_back">pop_back</a>(&<b>mut</b> other));
    <a href="Vector.md#0x1_Vector_destroy_empty">destroy_empty</a>(other);
}
</code></pre>



</details>

<a name="0x1_Vector_is_empty"></a>

## Function `is_empty`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_is_empty">is_empty</a>&lt;Element&gt;(v: &vector&lt;Element&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_is_empty">is_empty</a>&lt;Element&gt;(v: &vector&lt;Element&gt;): bool {
    <a href="Vector.md#0x1_Vector_length">length</a>(v) == 0
}
</code></pre>



</details>

<a name="0x1_Vector_contains"></a>

## Function `contains`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_contains">contains</a>&lt;Element&gt;(v: &vector&lt;Element&gt;, e: &Element): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_contains">contains</a>&lt;Element&gt;(v: &vector&lt;Element&gt;, e: &Element): bool {
    <b>let</b> i = 0;
    <b>let</b> len = <a href="Vector.md#0x1_Vector_length">length</a>(v);
    <b>while</b> (i &lt; len) {
        <b>if</b> (<a href="Vector.md#0x1_Vector_borrow">borrow</a>(v, i) == e) <b>return</b> <b>true</b>;
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="0x1_Vector_index_of"></a>

## Function `index_of`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_index_of">index_of</a>&lt;Element&gt;(v: &vector&lt;Element&gt;, e: &Element): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_index_of">index_of</a>&lt;Element&gt;(v: &vector&lt;Element&gt;, e: &Element): (bool, u64) {
    <b>let</b> i = 0;
    <b>let</b> len = <a href="Vector.md#0x1_Vector_length">length</a>(v);
    <b>while</b> (i &lt; len) {
        <b>if</b> (<a href="Vector.md#0x1_Vector_borrow">borrow</a>(v, i) == e) <b>return</b> (<b>true</b>, i);
        i = i + 1;
    };
    (<b>false</b>, 0)
}
</code></pre>



</details>

<a name="0x1_Vector_remove"></a>

## Function `remove`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_remove">remove</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, i: u64): Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_remove">remove</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, i: u64): Element {
    <b>let</b> len = <a href="Vector.md#0x1_Vector_length">length</a>(v);
    // i out of bounds; <b>abort</b>
    <b>if</b> (i &gt;= len) <b>abort</b> <a href="Vector.md#0x1_Vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>;

    len = len - 1;
    <b>while</b> (i &lt; len) <a href="Vector.md#0x1_Vector_swap">swap</a>(v, i, { i = i + 1; i });
    <a href="Vector.md#0x1_Vector_pop_back">pop_back</a>(v)
}
</code></pre>



</details>

<a name="0x1_Vector_swap_remove"></a>

## Function `swap_remove`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_swap_remove">swap_remove</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, i: u64): Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_swap_remove">swap_remove</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, i: u64): Element {
    <b>let</b> last_idx = <a href="Vector.md#0x1_Vector_length">length</a>(v) - 1;
    <a href="Vector.md#0x1_Vector_swap">swap</a>(v, i, last_idx);
    <a href="Vector.md#0x1_Vector_pop_back">pop_back</a>(v)
}
</code></pre>



</details>

<a name="0x1_Vector_split"></a>

## Function `split`



<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_split">split</a>&lt;Element: <b>copyable</b>&gt;(v: &vector&lt;Element&gt;, sub_len: u64): vector&lt;vector&lt;Element&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_split">split</a>&lt;Element: <b>copyable</b>&gt;(v: &vector&lt;Element&gt;, sub_len: u64): vector&lt;vector&lt;Element&gt;&gt; {
    <b>let</b> result = <a href="Vector.md#0x1_Vector_empty">empty</a>&lt;vector&lt;Element&gt;&gt;();
    <b>let</b> len = <a href="Vector.md#0x1_Vector_length">length</a>(v) / sub_len;

    <b>let</b> rem = 0;
    <b>if</b> (len * sub_len &lt; <a href="Vector.md#0x1_Vector_length">length</a>(v)) {
        rem = <a href="Vector.md#0x1_Vector_length">length</a>(v) - len * sub_len;
    };

    <b>let</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> sub = <a href="Vector.md#0x1_Vector_empty">empty</a>&lt;Element&gt;();
        <b>let</b> j = 0;
        <b>while</b> (j &lt; sub_len) {
            <b>let</b> index = sub_len * i + j;
            <a href="Vector.md#0x1_Vector_push_back">push_back</a>(&<b>mut</b> sub, *<a href="Vector.md#0x1_Vector_borrow">borrow</a>(v, index));
            j = j + 1;
        };
        <a href="Vector.md#0x1_Vector_push_back">push_back</a>&lt;vector&lt;Element&gt;&gt;(&<b>mut</b> result, sub);
        i = i + 1;
    };

    <b>if</b> (rem &gt; 0) {
        <b>let</b> sub = <a href="Vector.md#0x1_Vector_empty">empty</a>&lt;Element&gt;();
        <b>let</b> index = <a href="Vector.md#0x1_Vector_length">length</a>(v) - rem;
        <b>while</b> (index &lt; <a href="Vector.md#0x1_Vector_length">length</a>(v)) {
            <a href="Vector.md#0x1_Vector_push_back">push_back</a>(&<b>mut</b> sub, *<a href="Vector.md#0x1_Vector_borrow">borrow</a>(v, index));
            index = index + 1;
        };
        <a href="Vector.md#0x1_Vector_push_back">push_back</a>&lt;vector&lt;Element&gt;&gt;(&<b>mut</b> result, sub);
    };
    result
}
</code></pre>



</details>

<a name="@Specification_1"></a>

## Specification



<pre><code><b>pragma</b> verify;
<b>pragma</b> aborts_if_is_strict;
</code></pre>



<a name="@Specification_1_length"></a>

### Function `length`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_length">length</a>&lt;Element&gt;(v: &vector&lt;Element&gt;): u64
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a name="@Specification_1_borrow"></a>

### Function `borrow`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_borrow">borrow</a>&lt;Element&gt;(v: &vector&lt;Element&gt;, i: u64): &Element
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a name="@Specification_1_singleton"></a>

### Function `singleton`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_singleton">singleton</a>&lt;Element&gt;(e: Element): vector&lt;Element&gt;
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == <a href="Vector.md#0x1_Vector_spec_singleton">spec_singleton</a>(e);
</code></pre>




<a name="0x1_Vector_spec_singleton"></a>


<pre><code><b>define</b> <a href="Vector.md#0x1_Vector_spec_singleton">spec_singleton</a>&lt;Element&gt;(e: Element): vector&lt;Element&gt; {
    singleton_vector(e)
}
</code></pre>



<a name="@Specification_1_reverse"></a>

### Function `reverse`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_reverse">reverse</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a name="@Specification_1_append"></a>

### Function `append`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_append">append</a>&lt;Element&gt;(lhs: &<b>mut</b> vector&lt;Element&gt;, other: vector&lt;Element&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a name="@Specification_1_is_empty"></a>

### Function `is_empty`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_is_empty">is_empty</a>&lt;Element&gt;(v: &vector&lt;Element&gt;): bool
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a name="@Specification_1_contains"></a>

### Function `contains`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_contains">contains</a>&lt;Element&gt;(v: &vector&lt;Element&gt;, e: &Element): bool
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a name="@Specification_1_index_of"></a>

### Function `index_of`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_index_of">index_of</a>&lt;Element&gt;(v: &vector&lt;Element&gt;, e: &Element): (bool, u64)
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a name="@Specification_1_remove"></a>

### Function `remove`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_remove">remove</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, i: u64): Element
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a name="@Specification_1_swap_remove"></a>

### Function `swap_remove`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_swap_remove">swap_remove</a>&lt;Element&gt;(v: &<b>mut</b> vector&lt;Element&gt;, i: u64): Element
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a name="@Specification_1_split"></a>

### Function `split`


<pre><code><b>public</b> <b>fun</b> <a href="Vector.md#0x1_Vector_split">split</a>&lt;Element: <b>copyable</b>&gt;(v: &vector&lt;Element&gt;, sub_len: u64): vector&lt;vector&lt;Element&gt;&gt;
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
<b>aborts_if</b> sub_len == 0;
</code></pre>



<a name="@Module_specifications_2"></a>

### Module specifications


Auxiliary function to check whether a vector contains an element.


<a name="0x1_Vector_spec_contains"></a>


<pre><code><b>define</b> <a href="Vector.md#0x1_Vector_spec_contains">spec_contains</a>&lt;Element&gt;(v: vector&lt;Element&gt;, e: Element): bool {
    <b>exists</b> x in v: x == e
}
</code></pre>


Auxiliary function to check if <code>v1</code> is equal to the result of adding <code>e</code> at the end of <code>v2</code>


<a name="0x1_Vector_eq_push_back"></a>


<pre><code><b>define</b> <a href="Vector.md#0x1_Vector_eq_push_back">eq_push_back</a>&lt;Element&gt;(v1: vector&lt;Element&gt;, v2: vector&lt;Element&gt;, e: Element): bool {
    len(v1) == len(v2) + 1 &&
    v1[len(v1)-1] == e &&
    v1[0..len(v1)-1] == v2[0..len(v2)]
}
</code></pre>


Auxiliary function to check if <code>v</code> is equal to the result of concatenating <code>v1</code> and <code>v2</code>


<a name="0x1_Vector_eq_append"></a>


<pre><code><b>define</b> <a href="Vector.md#0x1_Vector_eq_append">eq_append</a>&lt;Element&gt;(v: vector&lt;Element&gt;, v1: vector&lt;Element&gt;, v2: vector&lt;Element&gt;): bool {
    len(v) == len(v1) + len(v2) &&
    v[0..len(v1)] == v1 &&
    v[len(v1)..len(v)] == v2
}
<a name="0x1_Vector_eq_pop_front"></a>
<b>define</b> <a href="Vector.md#0x1_Vector_eq_pop_front">eq_pop_front</a>&lt;Element&gt;(v1: vector&lt;Element&gt;, v2: vector&lt;Element&gt;): bool {
    len(v1) + 1 == len(v2) &&
    v1 == v2[1..len(v2)]
}
</code></pre>
