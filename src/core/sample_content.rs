//! Sample content templates for development data generation
//!
//! This module provides template content generators for creating realistic
//! course materials including lectures, assignments, and study materials.

use super::dev_data_generator::Course;

/// Template generator for course information files
pub struct CourseInfoTemplate;

impl CourseInfoTemplate {
    pub fn generate(course: &Course) -> String {
        format!(
            r#"= {}: {}

== Course Information
- *Course Code*: {}
- *Credits*: {}
- *Semester*: {}

== Description
{}

== Learning Objectives
- Understand fundamental concepts
- Apply theoretical knowledge to practical problems
- Develop analytical and problem-solving skills
- Work effectively in team environments

== Assessment
- Assignments: 40%
- Midterm: 25%
- Final Exam: 35%

== Schedule
Regular lectures on Mondays and Wednesdays, with exercise sessions on Fridays.

#pagebreak()

== Course Notes
This document serves as the main course overview. Individual lecture notes and assignments are stored in separate files.
"#,
            course.code,
            course.name,
            course.code,
            course.credits,
            course.semester,
            course.description
        )
    }
}

/// Template generator for lecture notes
pub struct LectureTemplate;

impl LectureTemplate {
    pub fn generate(lecture_num: usize, topic: &str, course: &Course, date: &str) -> String {
        format!(
            r#"= Lecture {}: {} 
*Course*: {} - {} \
*Date*: {} \
*Topic*: {}

== Overview
Today's lecture covers {} with focus on practical applications and theoretical foundations.

== Key Concepts

=== Main Topic: {}
- Important concept A: detailed explanation with examples
- Important concept B: relationship to previous material  
- Important concept C: applications and use cases

=== Secondary Topics
- Supporting concept 1
- Supporting concept 2
- Integration with course material

== Examples

```python
# Example code for {}
def example_function():
    # Implementation details
    pass
```

== Practice Problems
1. Problem related to {}
2. Application exercise
3. Theoretical question

== Summary
- Key takeaway 1
- Key takeaway 2
- Connection to next lecture

== References
- Course textbook, Chapter {}
- Additional reading materials
- Online resources

#pagebreak()
"#,
            lecture_num,
            topic,
            course.code,
            course.name,
            date,
            topic,
            topic,
            topic,
            topic,
            topic,
            lecture_num % 15 + 1
        )
    }
}

/// Template generator for assignments
pub struct AssignmentTemplate;

impl AssignmentTemplate {
    pub fn generate(
        assignment_num: usize,
        assignment_type: &str,
        course: &Course,
        due_date: &str,
        points: i32,
    ) -> String {
        format!(
            r#"= Assignment {}: {} Assignment
*Course*: {} - {} \
*Due Date*: {} \
*Points*: {} points \
*Type*: {}

== Instructions
Complete the following tasks related to the course material. Show all work and provide clear explanations for your solutions.

== Problem 1: Fundamental Concepts (25 points)
Explain the key concepts covered in recent lectures and demonstrate your understanding through examples.

=== Requirements:
- Clear explanation of concepts
- Relevant examples
- Proper formatting and documentation

== Problem 2: Practical Application (35 points)
Apply the theoretical knowledge to solve a practical problem.

```
// Your solution here
```

=== Submission Requirements:
- Working code with comments
- Test cases and validation
- Performance analysis if applicable

== Problem 3: Analysis and Discussion (25 points)
Analyze the given scenario and provide thoughtful discussion of alternatives and trade-offs.

== Problem 4: Extension (15 points)
Research and implement an extension or improvement to the basic solution.

== Submission Guidelines
- Submit all files in a single archive
- Include a README with instructions
- Ensure code compiles and runs
- Follow coding standards discussed in class

== Grading Criteria
- Correctness: 60%
- Code Quality: 20%
- Documentation: 15%
- Creativity: 5%

== Resources
- Course slides and notes
- Recommended textbooks
- Online documentation (cite sources)

*Note*: Late submissions will be penalized according to the course policy.
"#,
            assignment_num,
            assignment_type,
            course.code,
            course.name,
            due_date,
            points,
            assignment_type
        )
    }
}

/// Template generator for study materials
pub struct StudyMaterialsTemplate;

impl StudyMaterialsTemplate {
    pub fn generate_summary(course: &Course) -> String {
        format!(
            r#"= Course Summary: {}

== Quick Reference
*Course*: {} - {} \
*Credits*: {} \
*Semester*: {}

== Key Topics Covered
- Topic 1: Fundamental concepts and principles
- Topic 2: Advanced applications and theory
- Topic 3: Practical implementations
- Topic 4: Analysis and optimization
- Topic 5: Integration and synthesis

== Important Formulas
// Key formulas and equations used throughout the course

== Definitions
/ *Term 1*: Definition and explanation
/ *Term 2*: Definition with examples
/ *Term 3*: Mathematical or technical definition

== Common Patterns
- Design pattern A
- Algorithm approach B
- Problem-solving strategy C

== Exam Preparation
=== Study Checklist
- [ ] Review all lecture notes
- [ ] Complete practice problems
- [ ] Understand key concepts
- [ ] Review assignments and feedback

=== Important Dates
- Midterm: TBD
- Final Exam: TBD
- Assignment deadlines: See individual assignments
"#,
            course.name, course.code, course.name, course.credits, course.semester
        )
    }

    pub fn generate_cheat_sheet(course: &Course) -> String {
        format!(
            r#"= {} Cheat Sheet

== Quick Reference Card
*Course*: {} \
*Key Concepts*: Essential formulas, algorithms, and patterns

== Formulas
```
// Important formulas here
F = ma
E = mc²
```

== Algorithms
=== Algorithm 1
```python
def important_algorithm(input):
    # Step-by-step implementation
    return result
```

=== Algorithm 2
```python
def another_algorithm(data):
    # Another key algorithm
    return processed_data
```

== Common Patterns
- Pattern A: When to use and how
- Pattern B: Implementation details
- Pattern C: Performance considerations

== Troubleshooting
/ *Problem*: Solution approach
/ *Common Error*: How to fix
/ *Performance Issue*: Optimization tips
"#,
            course.name, course.code
        )
    }

    pub fn generate_exam_notes(course: &Course) -> String {
        format!(
            r#"= Exam Preparation: {}

== Exam Format
- Duration: 3 hours
- Format: Written + Practical
- Materials: Calculator allowed
- Coverage: All course material

== Study Strategy
=== Week 1-2 Before Exam
- Review all lecture notes systematically
- Work through practice problems
- Identify weak areas for focused study

=== Final Week
- Practice past exams
- Review key formulas and concepts
- Prepare cheat sheet (if allowed)

== Key Topics for Exam
=== High Priority
- Core concepts from lectures 1-5
- Major algorithms and their complexity
- Problem-solving techniques

=== Medium Priority
- Advanced applications
- Integration topics
- Case studies

=== Review Topics
- Basic foundations (assumed knowledge)
- Supplementary materials

== Practice Problems
=== Type A Problems
Focus on fundamental understanding and direct application.

=== Type B Problems
Require synthesis of multiple concepts and creative problem-solving.

=== Type C Problems
Advanced analysis and optimization challenges.

== Time Management
- Read all questions first (5 min)
- Start with confident topics (60 min)
- Tackle challenging problems (90 min)
- Review and check work (25 min)
"#,
            course.name
        )
    }
}

/// Get lecture topics for a specific course
pub fn get_lecture_topics(course_code: &str) -> Vec<String> {
    match course_code {
        "02101" => vec![
            "Introduction to Programming".to_string(),
            "Variables and Data Types".to_string(),
            "Control Structures".to_string(),
            "Functions and Modules".to_string(),
            "Data Structures".to_string(),
            "File I/O".to_string(),
            "Error Handling".to_string(),
            "Object-Oriented Programming".to_string(),
            "Testing and Debugging".to_string(),
            "Best Practices".to_string(),
        ],
        "02102" => vec![
            "Algorithm Analysis".to_string(),
            "Arrays and Lists".to_string(),
            "Stacks and Queues".to_string(),
            "Linked Lists".to_string(),
            "Trees and Binary Trees".to_string(),
            "Hash Tables".to_string(),
            "Sorting Algorithms".to_string(),
            "Searching Algorithms".to_string(),
            "Graph Algorithms".to_string(),
            "Dynamic Programming".to_string(),
        ],
        "02105" => vec![
            "Advanced Data Structures".to_string(),
            "Graph Theory".to_string(),
            "Network Flows".to_string(),
            "String Algorithms".to_string(),
            "Computational Geometry".to_string(),
            "Approximation Algorithms".to_string(),
            "Randomized Algorithms".to_string(),
            "Parallel Algorithms".to_string(),
            "Advanced Topics".to_string(),
            "Research Frontiers".to_string(),
        ],
        "02180" => vec![
            "Introduction to AI".to_string(),
            "Search Algorithms".to_string(),
            "Knowledge Representation".to_string(),
            "Machine Learning Basics".to_string(),
            "Neural Networks".to_string(),
            "Natural Language Processing".to_string(),
            "Computer Vision".to_string(),
            "Robotics".to_string(),
            "Ethics in AI".to_string(),
            "Future of AI".to_string(),
        ],
        _ => vec![
            "Introduction".to_string(),
            "Fundamental Concepts".to_string(),
            "Core Theory".to_string(),
            "Practical Applications".to_string(),
            "Advanced Topics".to_string(),
            "Case Studies".to_string(),
            "Integration".to_string(),
            "Optimization".to_string(),
            "Best Practices".to_string(),
            "Future Directions".to_string(),
        ],
    }
}
