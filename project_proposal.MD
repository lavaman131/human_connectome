# Project Proposal

1. What data set are you planning to use and why it is interesting to you?

I am planning to use [UCLA's Autism brain connectome dataset](http://umcd.humanconnectomeproject.org/umcd/default/browse_studies). Brain connectomics or brain network research has rapidly expanded using functional MRI (fMRI) and diffusion-weighted MRI (dwMRI). A common product of these varied analyses is a connectivity matrix (CM). A CM stores the connection strength between any two regions ("nodes") in a brain network. This format is useful for several reasons: (1) it is highly distilled, with minimal data size and complexity, (2) graph theory can be applied to characterize the network's topology, and (3) it retains sufficient information to capture individual differences such as age, gender, intelligence quotient (IQ), or disease state. This last point particularly interests me since I have passion for creating algorithms that contribute to improving the quality of life for others.

2. What is the problem you are solving or the question you are asking? Show that you have thought about it and share your insights.

I would like to analyze networks of functional correlations between the human brain regions using graph theoretic and combinatorial algorithmic tools to identify changes in their structure caused by Autism. This idea was inspired by Professor Chatterjee's PhD research in analyzing graphs to identify changes in structure caused by Attention Deficit Hyperactivity Disorder (ADHD).

3. What are the steps/components needed to accomplish the project? Specify the milestones with approximate dates you will accomplish them and how you plan to test the individual components.

Rough Schedule:

* November 17th - Conduct preliminary research on graph algorithms to approach problems Detecting network anomalies using [Forman–Ricci curvature and a case study for human brain networks](https://www.nature.com/articles/s41598-021-87587-z)
* November 22th - Start initial implementation of graph data structure
  * Load graph data structure
  * Find the number of n-sided polygons in a graph (up to 5 sides)
* November 29th - Have initial working algorithm for finding Forman–Ricci curvature of a graph
* December 6th - Find statistical results/significance of findings (first-order statistics of the normalized curvatures differences and the edge weight differences over all pairs of nodes in the disease and the control network)
* December 10th - Summarize statistical results in formal writeup 
* December 15th - Finalize Project and Submit 

I plan to test the individual components of my project by periodically checking in with Professor Kontothanassis as well as conducting automated tests within a Rust module.